use crate::adapter::VibeID;
use crate::env::resolve_state_dir;
use crate::error::Result;
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

    pub fn load(&self) -> Result<()> {
        if self.state_file.exists() {
            let content = fs::read_to_string(&self.state_file)?;
            let mut panes = self.panes.lock().unwrap();
            *panes = serde_json::from_str(&content).unwrap_or_default();
        }
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let panes = self.panes.lock().unwrap();
        let content = serde_json::to_string_pretty(&*panes)?;
        
        // Atomic write
        let tmp_file = self.state_file.with_extension("tmp");
        fs::write(&tmp_file, content)?;
        fs::rename(tmp_file, &self.state_file)?;
        
        Ok(())
    }

    #[allow(dead_code)]
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
}
