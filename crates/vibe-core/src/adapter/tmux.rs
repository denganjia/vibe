use crate::adapter::{SplitDirection, WindowTarget, TerminalAdapter, TerminalMetadata, VibeID};
use crate::error::{Result, VibeError};
use std::process::Command;

pub struct TmuxAdapter;

impl TerminalAdapter for TmuxAdapter {
    fn spawn(&self, target: WindowTarget, command: Option<&str>, env_vars: std::collections::HashMap<String, String>) -> Result<VibeID> {
        let mut cmd = Command::new("tmux");
        
        match target {
            WindowTarget::Pane(direction) => {
                cmd.args(["split-window", "-P", "-F", "#{pane_id}"]);
                match direction {
                    SplitDirection::Horizontal => cmd.arg("-h"),
                    SplitDirection::Vertical => cmd.arg("-v"),
                };
            }
            WindowTarget::Tab => {
                cmd.args(["new-window", "-P", "-F", "#{pane_id}"]);
            }
        }

        for (k, v) in env_vars {
            cmd.arg("-e").arg(format!("{}={}", k, v));
        }

        if let Some(c) = command {
            cmd.arg(format!("{}; exec bash", c));
        }

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux spawn failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let pane_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(pane_id)
    }

    fn send_keys(&self, target_id: &VibeID, keys: &str) -> Result<()> {
        self.inject_text(target_id, keys)?;
        self.inject_raw(target_id, "\r")
    }

    fn inject_raw(&self, target_id: &VibeID, text: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["send-keys", "-t", target_id, "-l", text])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux send-keys (literal) failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn close(&self, target_id: &VibeID) -> Result<()> {
        let output = Command::new("tmux")
            .args(["kill-pane", "-t", target_id])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux kill-pane failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn get_metadata(&self) -> Result<TerminalMetadata> {
        let current_pane_id = std::env::var("TMUX_PANE").map_err(|_| {
            VibeError::TerminalDetectionFailed("TMUX_PANE environment variable not set".into())
        })?;

        let output = Command::new("tmux")
            .args([
                "list-panes",
                "-a",
                "-F",
                "#{pane_id},#{window_id},#{session_name}",
            ])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux list-panes failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 && parts[0] == current_pane_id {
                return Ok(TerminalMetadata {
                    pane_id: parts[0].to_string(),
                    window_id: parts[1].to_string(),
                    session_id: parts[2].to_string(),
                });
            }
        }

        Err(VibeError::TerminalDetectionFailed(format!(
            "Current Tmux pane {} not found in list-panes",
            current_pane_id
        )))
    }

    fn list_all_physical_ids(&self) -> Result<Vec<String>> {
        let output = Command::new("tmux")
            .args(["list-panes", "-a", "-F", "#{pane_id}"])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux list-panes failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|s| s.to_string()).collect())
    }

    fn focus(&self, target_id: &VibeID) -> Result<()> {
        let output = Command::new("tmux")
            .args(["select-pane", "-t", target_id])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux select-pane failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }
}
