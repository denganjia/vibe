use crate::adapter::VibeID;
use crate::env::resolve_state_dir;
use crate::error::Result;
use crate::ipc::protocol::RegisterInfo;
use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
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
        let mut conn = Connection::open(db_path)?;
        
        let migrations = Self::migrations();
        migrations.to_latest(&mut conn).map_err(|e| crate::error::VibeError::Internal(format!("Migration failed: {}", e)))?;
        
        Ok(Self { conn })
    }

    fn migrations() -> Migrations<'static> {
        Migrations::new(vec![
            M::up("CREATE TABLE IF NOT EXISTS panes (
                vibe_id TEXT PRIMARY KEY,
                physical_id TEXT NOT NULL,
                terminal_type TEXT NOT NULL
            );"),
            M::up("ALTER TABLE panes ADD COLUMN role TEXT;
                   ALTER TABLE panes ADD COLUMN status TEXT;
                   ALTER TABLE panes ADD COLUMN summary TEXT;
                   ALTER TABLE panes ADD COLUMN pid INTEGER;
                   ALTER TABLE panes ADD COLUMN last_heartbeat_at DATETIME;
                   ALTER TABLE panes ADD COLUMN cwd TEXT;
                   ALTER TABLE panes ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;"),
            M::up("ALTER TABLE panes ADD COLUMN approval_status TEXT DEFAULT 'none';
                   ALTER TABLE panes ADD COLUMN plan_path TEXT;
                   ALTER TABLE panes ADD COLUMN rejection_reason TEXT;"),
        ])
    }

    pub fn from_conn(mut conn: Connection) -> Self {
        let migrations = Self::migrations();
        let _ = migrations.to_latest(&mut conn);
        Self { conn }
    }

    pub fn save_pane(&self, vibe_id: &VibeID, physical_id: &str, terminal_type: &str, cwd: Option<String>) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO panes (vibe_id, physical_id, terminal_type, cwd) VALUES (?1, ?2, ?3, ?4)",
            params![vibe_id, physical_id, terminal_type, cwd],
        )?;
        Ok(())
    }

    pub fn register_pane(&self, info: &RegisterInfo) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO panes (vibe_id, physical_id, terminal_type, role, pid, status, cwd) VALUES (?1, ?2, ?3, ?4, ?5, 'registered', ?6)",
            params![info.vibe_id, info.physical_id, info.terminal_type, info.role, info.pid, info.cwd],
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

    pub fn list_active_panes(&self) -> Result<Vec<(VibeID, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>> {
        let mut stmt = self.conn.prepare("SELECT vibe_id, physical_id, terminal_type, role, status, summary, cwd, approval_status, plan_path, rejection_reason FROM panes")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
            ))
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
        let store = StateStore::from_conn(conn);
        
        let v_id = "vibe-1".to_string();
        store.save_pane(&v_id, "phys-1", "wezterm", None)?;
        
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
