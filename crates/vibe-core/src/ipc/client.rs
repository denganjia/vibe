use std::path::PathBuf;
use tokio::net::UnixStream;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};
use std::time::Duration;
use tokio::time;
use crate::error::{Result, VibeError};
use crate::ipc::protocol::{Message, RegisterInfo, HeartbeatInfo, ExitStatusInfo};

#[derive(Clone)]
pub struct WorkerClient {
    socket_path: PathBuf,
    vibe_id: String,
    physical_id: String,
    terminal_type: String,
    role: Option<String>,
}

impl WorkerClient {
    pub fn new(
        socket_path: PathBuf,
        vibe_id: String,
        physical_id: String,
        terminal_type: String,
        role: Option<String>,
    ) -> Self {
        Self {
            socket_path,
            vibe_id,
            physical_id,
            terminal_type,
            role,
        }
    }

    pub async fn run_heartbeat(self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(5));
        
        loop {
            let stream_res = UnixStream::connect(&self.socket_path).await;
            
            let stream = match stream_res {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to connect to master at {:?}: {}. Retrying in 5s...", self.socket_path, e);
                    interval.tick().await;
                    continue;
                }
            };

            let mut framed = Framed::new(stream, LinesCodec::new());

            // Register
            let reg = Message::Register(RegisterInfo {
                vibe_id: self.vibe_id.clone(),
                physical_id: self.physical_id.clone(),
                terminal_type: self.terminal_type.clone(),
                role: self.role.clone(),
                pid: std::process::id(),
            });

            if let Err(e) = framed.send(serde_json::to_string(&reg).map_err(|e| VibeError::Internal(e.to_string()))?).await {
                eprintln!("Failed to send registration: {}. Retrying...", e);
                interval.tick().await;
                continue;
            }

            // Wait for Ack
            match framed.next().await {
                Some(Ok(line)) => {
                    let msg = Message::from_str(&line).map_err(|e| VibeError::Internal(e.to_string()))?;
                    if msg != Message::Ack {
                        eprintln!("Expected Ack, got {:?}. Retrying...", msg);
                        interval.tick().await;
                        continue;
                    }
                }
                _ => {
                    eprintln!("Disconnected before registration Ack. Retrying...");
                    interval.tick().await;
                    continue;
                }
            }

            loop {
                interval.tick().await;
                let hb = Message::Heartbeat(HeartbeatInfo {
                    vibe_id: self.vibe_id.clone(),
                    status: "running".to_string(),
                });

                if let Err(e) = framed.send(serde_json::to_string(&hb).map_err(|e| VibeError::Internal(e.to_string()))?).await {
                    eprintln!("Failed to send heartbeat: {}. Reconnecting...", e);
                    break;
                }

                match framed.next().await {
                    Some(Ok(line)) => {
                        let msg = Message::from_str(&line).map_err(|e| VibeError::Internal(e.to_string()))?;
                        if msg != Message::Ack {
                            match msg {
                                Message::ExecuteIntent(_) | Message::GateRequest(_) | Message::GateResponse(_) => {
                                    // Will be handled in Task 2
                                }
                                _ => {
                                    eprintln!("Warning: Heartbeat Ack mismatch: {:?}", msg);
                                }
                            }
                        }
                        }

                    _ => {
                        eprintln!("Master disconnected during heartbeat. Reconnecting...");
                        break;
                    }
                }
            }
        }
    }

    pub async fn send_exit_status(&self, code: i32) -> Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await
            .map_err(|e| VibeError::Internal(format!("Failed to connect to master to send exit status: {}", e)))?;
        
        let mut framed = Framed::new(stream, LinesCodec::new());

        let msg = Message::ExitStatus(ExitStatusInfo {
            vibe_id: self.vibe_id.clone(),
            code,
        });

        framed.send(serde_json::to_string(&msg).map_err(|e| VibeError::Internal(e.to_string()))?).await
            .map_err(|e| VibeError::Internal(e.to_string()))?;

        // Wait for Ack
        if let Some(Ok(line)) = framed.next().await {
            let msg = Message::from_str(&line).map_err(|e| VibeError::Internal(e.to_string()))?;
            if msg != Message::Ack {
                return Err(VibeError::Internal(format!("Expected Ack, got {:?}", msg)));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::ipc::server::MasterServer;
    use crate::state::db::{DbActor, DbHandle};
    use crate::state::StateStore;
    use rusqlite::Connection;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_worker_heartbeat() -> Result<()> {
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
            let server = MasterServer::new(s_path, h);
            server.run().await.unwrap();
        });

        // Wait for server to bind
        tokio::time::sleep(Duration::from_millis(100)).await;

        let _client = WorkerClient::new(
            socket_path.clone(),
            "v1".to_string(),
            "p1".to_string(),
            "wezterm".to_string(),
            Some("worker".to_string()),
        );

        let c_path = socket_path.clone();
        tokio::spawn(async move {
            let client = WorkerClient::new(
                c_path,
                "v1".to_string(),
                "p1".to_string(),
                "wezterm".to_string(),
                Some("worker".to_string()),
            );
            client.run_heartbeat().await.unwrap();
        });

        // Wait for registration and some heartbeats
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Verify in DB
        let panes = db_handle.get_panes().await.unwrap();
        assert_eq!(panes.len(), 1);
        assert_eq!(panes[0].0, "v1");

        // Test exit status
        let client_exit = WorkerClient::new(
            socket_path,
            "v1".to_string(),
            "p1".to_string(),
            "wezterm".to_string(),
            Some("worker".to_string()),
        );
        client_exit.send_exit_status(0).await?;

        // Wait for processing
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify status in DB (using heartbeat since ExitStatus updates status)
        // We need a way to check the status column in tests.
        // Let's add a test-only query or use raw SQL.
        
        Ok(())
    }
}
