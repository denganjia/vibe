use crate::adapter::VibeID;
use crate::env::resolve_state_dir;
use crate::error::Result;
use crate::ipc::protocol::RegisterInfo;
use rusqlite::{params, Connection};
use std::fs;

pub mod db;

pub struct StateStore {
    conn: Connection,
}

impl StateStore {
    pub fn new() -> Result<Self> {
        let state_dir = resolve_state_dir()?;
        fs::create_dir_all(&state_dir)?;
        let db_path = state_dir.join("state.db");
        let conn = Connection::open(db_path)?;
        
        // Initialize schema
        let schema = include_str!("schema.sql");
        conn.execute_batch(schema)?;
        
        Ok(Self { conn })
    }

    pub fn from_conn(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn save_pane(&self, vibe_id: &VibeID, physical_id: &str, terminal_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO panes (vibe_id, physical_id, terminal_type) VALUES (?1, ?2, ?3)",
            params![vibe_id, physical_id, terminal_type],
        )?;
        Ok(())
    }

    pub fn register_pane(&self, info: &RegisterInfo) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO panes (vibe_id, physical_id, terminal_type, role, pid, status) VALUES (?1, ?2, ?3, ?4, ?5, 'registered')",
            params![info.vibe_id, info.physical_id, info.terminal_type, info.role, info.pid],
        )?;
        Ok(())
    }

    pub fn update_heartbeat(&self, vibe_id: &str, status: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE panes SET status = ?1, last_heartbeat_at = CURRENT_TIMESTAMP WHERE vibe_id = ?2",
            params![status, vibe_id],
        )?;
        Ok(())
    }

    pub fn update_report(&self, vibe_id: &str, status: &str, summary: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE panes SET status = ?1, summary = ?2, last_heartbeat_at = CURRENT_TIMESTAMP WHERE vibe_id = ?3",
            params![status, summary, vibe_id],
        )?;
        Ok(())
    }

    pub fn get_pane(&self, vibe_id: &VibeID) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT physical_id FROM panes WHERE vibe_id = ?1")?;
        let mut rows = stmt.query(params![vibe_id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_vibe_id_by_physical_id(&self, physical_id: &str) -> Result<Option<VibeID>> {
        let mut stmt = self.conn.prepare("SELECT vibe_id FROM panes WHERE physical_id = ?1")?;
        let mut rows = stmt.query(params![physical_id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_active_panes(&self) -> Result<Vec<(VibeID, String, String, Option<String>, Option<String>, Option<String>)>> {
        let mut stmt = self.conn.prepare("SELECT vibe_id, physical_id, terminal_type, role, status, summary FROM panes")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
        })?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn remove_pane(&self, vibe_id: &VibeID) -> Result<()> {
        self.conn.execute("DELETE FROM panes WHERE vibe_id = ?1", params![vibe_id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_persistence() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let schema = include_str!("schema.sql");
        conn.execute_batch(schema)?;
        
        let store = StateStore { conn };
        
        let v_id = "vibe-1".to_string();
        store.save_pane(&v_id, "phys-1", "wezterm")?;
        
        let phys_id = store.get_pane(&v_id)?;
        assert_eq!(phys_id, Some("phys-1".to_string()));
        
        let active = store.list_active_panes()?;
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].0, v_id);
        
        store.remove_pane(&v_id)?;
        let phys_id_after = store.get_pane(&v_id)?;
        assert_eq!(phys_id_after, None);
        
        Ok(())
    }
}
