use crate::error::Result;
use crate::ipc::protocol::RegisterInfo;
use crate::state::{StateStore, VibeID};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum DbRequest {
    RegisterPane(RegisterInfo, oneshot::Sender<Result<()>>),
    UpdateHeartbeat(String, String, oneshot::Sender<Result<()>>), // vibe_id, status
    UpdateReport(String, String, String, oneshot::Sender<Result<()>>), // vibe_id, status, summary
    GetPanes(oneshot::Sender<Result<Vec<(VibeID, String, String, Option<String>, Option<String>, Option<String>)>>>),
}

pub struct DbActor {
    receiver: mpsc::Receiver<DbRequest>,
    store: StateStore,
}

impl DbActor {
    pub fn new(receiver: mpsc::Receiver<DbRequest>, store: StateStore) -> Self {
        Self { receiver, store }
    }

    pub async fn run(mut self) {
        while let Some(request) = self.receiver.recv().await {
            self.handle_request(request);
        }
    }

    fn handle_request(&mut self, request: DbRequest) {
        match request {
            DbRequest::RegisterPane(info, res_tx) => {
                let res = self.store.register_pane(&info);
                let _ = res_tx.send(res);
            }
            DbRequest::UpdateHeartbeat(vibe_id, status, res_tx) => {
                let res = self.store.update_heartbeat(&vibe_id, &status);
                let _ = res_tx.send(res);
            }
            DbRequest::UpdateReport(vibe_id, status, summary, res_tx) => {
                let res = self.store.update_report(&vibe_id, &status, &summary);
                let _ = res_tx.send(res);
            }
            DbRequest::GetPanes(res_tx) => {
                let res = self.store.list_active_panes();
                let _ = res_tx.send(res);
            }
        }
    }
}

#[derive(Clone)]
pub struct DbHandle {
    sender: mpsc::Sender<DbRequest>,
}

impl DbHandle {
    pub fn new(sender: mpsc::Sender<DbRequest>) -> Self {
        Self { sender }
    }

    pub async fn register_pane(&self, info: RegisterInfo) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(DbRequest::RegisterPane(info, tx))
            .await
            .map_err(|e| crate::error::VibeError::Internal(format!("Failed to send request: {}", e)))?;
        rx.await.map_err(|e| crate::error::VibeError::Internal(format!("Failed to receive response: {}", e)))?
    }

    pub async fn update_heartbeat(&self, vibe_id: String, status: String) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(DbRequest::UpdateHeartbeat(vibe_id, status, tx))
            .await
            .map_err(|e| crate::error::VibeError::Internal(format!("Failed to send request: {}", e)))?;
        rx.await.map_err(|e| crate::error::VibeError::Internal(format!("Failed to receive response: {}", e)))?
    }

    pub async fn update_report(&self, vibe_id: String, status: String, summary: String) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(DbRequest::UpdateReport(vibe_id, status, summary, tx))
            .await
            .map_err(|e| crate::error::VibeError::Internal(format!("Failed to send request: {}", e)))?;
        rx.await.map_err(|e| crate::error::VibeError::Internal(format!("Failed to receive response: {}", e)))?
    }

    pub async fn get_panes(&self) -> Result<Vec<(VibeID, String, String, Option<String>, Option<String>, Option<String>)>> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(DbRequest::GetPanes(tx))
            .await
            .map_err(|e| crate::error::VibeError::Internal(format!("Failed to send request: {}", e)))?;
        rx.await.map_err(|e| crate::error::VibeError::Internal(format!("Failed to receive response: {}", e)))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[tokio::test]
    async fn test_actor_concurrency() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let schema = include_str!("schema.sql");
        conn.execute_batch(schema)?;
        let store = StateStore::from_conn(conn);
        
        let (tx, rx) = mpsc::channel(10);
        let actor = DbActor::new(rx, store);
        
        let handle = DbHandle::new(tx);
        
        tokio::spawn(async move {
            actor.run().await;
        });
        
        let info = RegisterInfo {
            vibe_id: "v1".to_string(),
            physical_id: "p1".to_string(),
            terminal_type: "wezterm".to_string(),
            role: Some("worker".to_string()),
            pid: 1234,
        };
        
        handle.register_pane(info).await?;
        
        let panes = handle.get_panes().await?;
        assert_eq!(panes.len(), 1);
        assert_eq!(panes[0].0, "v1");
        
        handle.update_heartbeat("v1".to_string(), "running".to_string()).await?;
        
        Ok(())
    }
}
