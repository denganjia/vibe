use crate::adapter::{SplitDirection, TerminalAdapter, TerminalMetadata, VibeID};
use crate::error::{Result, VibeError};
use std::process::Command;

pub struct TmuxAdapter;

impl TerminalAdapter for TmuxAdapter {
    fn split(&self, direction: SplitDirection, _size: Option<u32>) -> Result<VibeID> {
        let mut cmd = Command::new("tmux");
        cmd.args(["split-window", "-P", "-F", "#{pane_id}"]);

        match direction {
            SplitDirection::Horizontal => cmd.arg("-h"),
            SplitDirection::Vertical => cmd.arg("-v"),
        };

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux split-window failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let pane_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(pane_id)
    }

    fn send_keys(&self, target_id: &VibeID, keys: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["send-keys", "-t", target_id])
            .arg(keys)
            .arg("ENTER")
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "Tmux send-keys failed: {}",
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
}
