use crate::error::Result;
use crate::env::resolve_bus_dir;
use crate::ipc::protocol::SignalInfo;
use serde_json::Value;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

pub struct FileBus;

impl FileBus {
    /// Sends a signal to the bus by writing a JSON file to .vibe/bus/
    pub fn send(signal_name: &str, payload: Value) -> Result<()> {
        let bus_dir = resolve_bus_dir()?;
        
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        let uuid = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let filename = format!("{}-{}.json", ts, uuid);
        let final_path = bus_dir.join(filename);
        let tmp_path = final_path.with_extension("tmp");

        let signal = SignalInfo {
            name: signal_name.to_string(),
            payload,
        };

        let content = serde_json::to_string_pretty(&signal)?;
        
        // Atomic write
        fs::write(&tmp_path, content)?;
        fs::rename(tmp_path, final_path)?;

        Ok(())
    }

    /// Receives a signal from the bus by polling the .vibe/bus/ directory.
    /// Matches the first signal with the given name, consumes it (deletes file), and returns the payload.
    pub fn recv(signal_name: &str, timeout_secs: u64) -> Result<Value> {
        let bus_dir = resolve_bus_dir()?;
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        while start.elapsed() < timeout {
            let entries = fs::read_dir(&bus_dir)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(signal) = serde_json::from_str::<SignalInfo>(&content) {
                            if signal.name == signal_name {
                                // Consume the signal
                                let _ = fs::remove_file(&path);
                                return Ok(signal.payload);
                            }
                        }
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(100));
        }

        Err(crate::error::VibeError::Internal(format!("Timeout waiting for signal: {}", signal_name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;

    #[test]
    #[ignore]
    fn test_bus_send_recv() -> Result<()> {
        let original_dir = env::current_dir()?;
        let dir = tempdir()?;
        env::set_current_dir(dir.path())?;
        
        // Create .vibe/bus dir implicitly via resolve_bus_dir
        
        let signal_name = "test_signal";
        let payload = serde_json::json!({"foo": "bar"});

        // Run recv in a separate thread
        let handle = thread::spawn({
            let name = signal_name.to_string();
            move || {
                FileBus::recv(&name, 2)
            }
        });

        // Give recv a moment to start
        thread::sleep(Duration::from_millis(100));

        FileBus::send(signal_name, payload.clone())?;

        let received_payload = handle.join().unwrap()?;
        assert_eq!(received_payload, payload);

        env::set_current_dir(original_dir)?;
        Ok(())
    }
}
