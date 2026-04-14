use std::path::PathBuf;
use tokio::net::{UnixListener, UnixStream};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};
use std::time::Duration;
use tokio::time::{self, Instant};
use crate::error::{Result, VibeError};
use crate::ipc::protocol::Message;
use crate::state::db::DbHandle;
use tokio::sync::mpsc;

pub struct MasterServer {
    socket_path: PathBuf,
    db_handle: DbHandle,
    idle_timeout: Duration,
}

impl MasterServer {
    pub fn new(socket_path: PathBuf, db_handle: DbHandle) -> Self {
        Self {
            socket_path,
            db_handle,
            idle_timeout: Duration::from_secs(600), // 10 minutes
        }
    }

    pub async fn run(self) -> Result<()> {
        // Cleanup stale socket
        if self.socket_path.exists() {
            tokio::fs::remove_file(&self.socket_path).await
                .map_err(|e| VibeError::Internal(format!("Failed to remove stale socket: {}", e)))?;
        }

        // Ensure parent directory exists
        if let Some(parent) = self.socket_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| VibeError::Internal(format!("Failed to create socket directory: {}", e)))?;
        }

        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| VibeError::Internal(format!("Failed to bind to socket {:?}: {}", self.socket_path, e)))?;

        let (activity_tx, mut activity_rx) = mpsc::channel::<()>(100);
        let (disconnect_tx, mut disconnect_rx) = mpsc::channel::<()>(100);
        let mut active_connections = 0;
        let mut last_activity = Instant::now();
        
        println!("Master server listening on {:?}", self.socket_path);

        loop {
            tokio::select! {
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, _)) => {
                            active_connections += 1;
                            let db = self.db_handle.clone();
                            let a_tx = activity_tx.clone();
                            let d_tx = disconnect_tx.clone();
                            tokio::spawn(async move {
                                let _ = a_tx.send(()).await;
                                if let Err(e) = handle_connection(stream, db, a_tx).await {
                                    eprintln!("Connection error: {}", e);
                                }
                                let _ = d_tx.send(()).await;
                            });
                            last_activity = Instant::now();
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                        }
                    }
                }
                _ = activity_rx.recv() => {
                    last_activity = Instant::now();
                }
                _ = disconnect_rx.recv() => {
                    active_connections -= 1;
                    last_activity = Instant::now();
                }
                _ = time::sleep(Duration::from_secs(30)) => {
                    if active_connections == 0 && last_activity.elapsed() >= self.idle_timeout {
                        println!("Idle timeout reached, shutting down master.");
                        break;
                    }
                }
            }
        }

        // Cleanup on exit
        if self.socket_path.exists() {
            let _ = tokio::fs::remove_file(&self.socket_path).await;
        }

        Ok(())
    }
}

async fn handle_connection(stream: UnixStream, db: DbHandle, activity_tx: mpsc::Sender<()>) -> Result<()> {
    let mut framed = Framed::new(stream, LinesCodec::new());

    while let Some(result) = framed.next().await {
        let _ = activity_tx.send(()).await;
        match result {
            Ok(line) => {
                let msg = Message::from_str(&line)?;
                match msg {
                    Message::Register(info) => {
                        db.register_pane(info).await?;
                        let ack = serde_json::to_string(&Message::Ack)?;
                        framed.send(ack).await
                            .map_err(|e| VibeError::Internal(e.to_string()))?;
                    }
                    Message::Heartbeat(info) => {
                        db.update_heartbeat(info.vibe_id, info.status).await?;
                        let ack = serde_json::to_string(&Message::Ack)?;
                        framed.send(ack).await
                            .map_err(|e| VibeError::Internal(e.to_string()))?;
                    }
                    Message::ExitStatus(info) => {
                        let status = format!("exited:{}", info.code);
                        db.update_heartbeat(info.vibe_id, status).await?;
                        let ack = serde_json::to_string(&Message::Ack)?;
                        framed.send(ack).await
                            .map_err(|e| VibeError::Internal(e.to_string()))?;
                    }
                    Message::Ack => {}
                }
            }
            Err(e) => {
                return Err(VibeError::Internal(format!("Codec error: {}", e)));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::state::db::{DbActor, DbHandle};
    use crate::state::StateStore;
    use rusqlite::Connection;
    use crate::ipc::protocol::RegisterInfo;

    #[tokio::test]
    async fn test_master_idle_timeout() -> Result<()> {
        let dir = tempdir().unwrap();
        let socket_path = dir.path().join("vibe.sock");
        
        let conn = Connection::open_in_memory().unwrap();
        // Load schema
        let schema = "CREATE TABLE IF NOT EXISTS panes (vibe_id TEXT PRIMARY KEY, physical_id TEXT, terminal_type TEXT, role TEXT, pid INTEGER, status TEXT, last_heartbeat_at DATETIME DEFAULT CURRENT_TIMESTAMP);";
        conn.execute_batch(schema).unwrap();
        let store = StateStore::from_conn(conn);
        let (db_tx, db_rx) = mpsc::channel(10);
        let actor = DbActor::new(db_rx, store);
        tokio::spawn(async move { actor.run().await; });
        let db_handle = DbHandle::new(db_tx);

        let server = MasterServer {
            socket_path: socket_path.clone(),
            db_handle,
            idle_timeout: Duration::from_millis(500),
        };

        let start = Instant::now();
        server.run().await?;
        assert!(start.elapsed() >= Duration::from_millis(500));
        assert!(!socket_path.exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_master_interaction() -> Result<()> {
        let dir = tempdir().unwrap();
        let socket_path = dir.path().join("vibe.sock");
        
        let conn = Connection::open_in_memory().unwrap();
        let schema = "CREATE TABLE IF NOT EXISTS panes (vibe_id TEXT PRIMARY KEY, physical_id TEXT, terminal_type TEXT, role TEXT, pid INTEGER, status TEXT, last_heartbeat_at DATETIME DEFAULT CURRENT_TIMESTAMP);";
        conn.execute_batch(schema).unwrap();
        let store = StateStore::from_conn(conn);
        let (db_tx, db_rx) = mpsc::channel(10);
        let actor = DbActor::new(db_rx, store);
        tokio::spawn(async move { actor.run().await; });
        let db_handle = DbHandle::new(db_tx);

        let s_path = socket_path.clone();
        let h = db_handle.clone();
        tokio::spawn(async move {
            let server = MasterServer {
                socket_path: s_path,
                db_handle: h,
                idle_timeout: Duration::from_secs(5),
            };
            server.run().await.unwrap();
        });

        // Wait for server to bind
        tokio::time::sleep(Duration::from_millis(100)).await;

        let stream = UnixStream::connect(&socket_path).await.unwrap();
        let mut framed = Framed::new(stream, LinesCodec::new());

        let reg = Message::Register(RegisterInfo {
            vibe_id: "v1".to_string(),
            physical_id: "p1".to_string(),
            terminal_type: "wezterm".to_string(),
            role: Some("worker".to_string()),
            pid: 1234,
        });

        framed.send(serde_json::to_string(&reg).unwrap()).await.unwrap();
        
        if let Some(Ok(line)) = framed.next().await {
            let msg = Message::from_str(&line).unwrap();
            assert_eq!(msg, Message::Ack);
        } else {
            panic!("Expected Ack");
        }

        // Verify in DB
        let panes = db_handle.get_panes().await.unwrap();
        assert_eq!(panes.len(), 1);
        assert_eq!(panes[0].0, "v1");

        Ok(())
    }
}
