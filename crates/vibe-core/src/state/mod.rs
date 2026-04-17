use crate::adapter::VibeID;
use crate::env::{resolve_state_dir, resolve_project_vibe_dir};
use crate::error::Result;
use crate::ipc::protocol::{RegisterInfo, WorkerState};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaneRecord {
    pub vibe_id: VibeID,
    pub physical_id: String,
    pub terminal_type: String,
    pub role: Option<String>,
    pub status: Option<String>,
    pub summary: Option<String>,
    pub pid: Option<u32>,
    pub cwd: Option<String>,
    pub last_heartbeat_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct StateStore {
    state_file: PathBuf,
    panes: Arc<Mutex<HashMap<VibeID, PaneRecord>>>,
}

struct LockGuard {
    path: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl StateStore {
    pub fn new() -> Result<Self> {
        let state_dir = resolve_state_dir()?;
        if !state_dir.exists() {
            fs::create_dir_all(&state_dir)?;
        }
        let state_file = state_dir.join("panes.json");
        
        Ok(Self {
            state_file,
            panes: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn load(&self) -> Result<()> {
        if self.state_file.exists() {
            let content = fs::read_to_string(&self.state_file)?;
            let mut panes = self.panes.lock().unwrap();
            *panes = serde_json::from_str(&content).unwrap_or_default();
        }
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let panes = self.panes.lock().unwrap();
        let content = serde_json::to_string_pretty(&*panes)?;
        
        // Atomic write
        let tmp_file = self.state_file.with_extension("tmp");
        fs::write(&tmp_file, content)?;
        fs::rename(tmp_file, &self.state_file)?;
        
        Ok(())
    }

    fn acquire_lock(&self) -> Result<LockGuard> {
        let lock_file = self.state_file.with_extension("lock");
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(5);

        while start.elapsed() < timeout {
            match fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&lock_file) {
                Ok(_) => return Ok(LockGuard { path: lock_file }),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => return Err(e.into()),
            }
        }
        Err(crate::error::VibeError::Internal("Timeout acquiring state lock".into()))
    }

    pub fn get_master_pane(&self) -> Result<Option<String>> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let panes = self.panes.lock().unwrap();
        let mut all: Vec<_> = panes.values().collect();
        all.sort_by_key(|r| r.created_at);
        Ok(all.first().map(|r| r.physical_id.clone()))
    }

    pub fn save_pane(&self, vibe_id: &VibeID, physical_id: &str, terminal_type: &str, cwd: Option<String>) -> Result<()> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let mut panes = self.panes.lock().unwrap();
        let record = PaneRecord {
            vibe_id: vibe_id.clone(),
            physical_id: physical_id.to_string(),
            terminal_type: terminal_type.to_string(),
            role: None,
            status: Some("spawned".to_string()),
            summary: None,
            pid: None,
            cwd,
            last_heartbeat_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
        };
        panes.insert(vibe_id.clone(), record);
        drop(panes);
        self.save()
    }

    pub fn register_pane(&self, info: RegisterInfo) -> Result<()> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let mut panes = self.panes.lock().unwrap();
        let record = PaneRecord {
            vibe_id: info.vibe_id.clone(),
            physical_id: info.physical_id,
            terminal_type: info.terminal_type,
            role: info.role,
            status: Some("registered".to_string()),
            summary: None,
            pid: Some(info.pid),
            cwd: info.cwd,
            last_heartbeat_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
        };
        panes.insert(info.vibe_id, record);
        drop(panes);
        self.save()
    }

    pub fn update_heartbeat(&self, vibe_id: String, status: String) -> Result<()> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let mut panes = self.panes.lock().unwrap();
        if let Some(record) = panes.get_mut(&vibe_id) {
            record.status = Some(status);
            record.last_heartbeat_at = Some(chrono::Utc::now());
        }
        drop(panes);
        self.save()
    }

    pub fn update_report(&self, vibe_id: String, status: String, summary: String) -> Result<()> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let mut panes = self.panes.lock().unwrap();
        if let Some(record) = panes.get_mut(&vibe_id) {
            record.status = Some(status);
            record.summary = Some(summary);
            record.last_heartbeat_at = Some(chrono::Utc::now());
        }
        drop(panes);
        self.save()
    }

    pub fn get_pane(&self, vibe_id: &VibeID) -> Result<Option<String>> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let panes = self.panes.lock().unwrap();
        Ok(panes.get(vibe_id).map(|r| r.physical_id.clone()))
    }

    pub fn get_vibe_id_by_physical_id(&self, physical_id: &str) -> Result<Option<VibeID>> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let panes = self.panes.lock().unwrap();
        for record in panes.values() {
            if record.physical_id == physical_id {
                return Ok(Some(record.vibe_id.clone()));
            }
        }
        Ok(None)
    }

    pub fn list_active_panes(&self) -> Result<Vec<WorkerState>> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let panes = self.panes.lock().unwrap();
        let mut results = Vec::new();
        
        for record in panes.values() {
            results.push(WorkerState {
                vibe_id: record.vibe_id.clone(),
                physical_id: record.physical_id.clone(),
                role: record.role.clone(),
                status: record.status.clone().unwrap_or_default(),
                summary: record.summary.clone().unwrap_or_default(),
                last_seen: record.last_heartbeat_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
                cwd: record.cwd.clone(),
            });
        }
        Ok(results)
    }

    pub fn remove_pane(&self, vibe_id: &VibeID) -> Result<()> {
        let _lock = self.acquire_lock()?;
        self.load()?;
        let mut panes = self.panes.lock().unwrap();
        panes.remove(vibe_id);
        drop(panes);
        self.save()
    }
}

pub fn ensure_project_vibe() -> Result<PathBuf> {
    let vibe_dir = resolve_project_vibe_dir()?;
    if !vibe_dir.exists() {
        fs::create_dir_all(&vibe_dir)?;
        fs::create_dir_all(vibe_dir.join("roles"))?;
        fs::create_dir_all(vibe_dir.join("state"))?;
        println!("Initialized .vibe directory in {:?}", vibe_dir);
    }
    Ok(vibe_dir)
}
