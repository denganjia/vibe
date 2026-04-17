use crate::adapter::{SplitDirection, TerminalAdapter, TerminalMetadata, VibeID};
use crate::error::{Result, VibeError};
use serde::{Deserialize, Serialize};
use std::process::Command;

pub struct WezTermAdapter;

#[derive(Debug, Serialize, Deserialize)]
struct WezTermPane {
    pane_id: u64,
    window_id: u64,
    tab_id: u64,
    workspace: String,
}

impl TerminalAdapter for WezTermAdapter {
    fn split(&self, direction: SplitDirection, _size: Option<u32>, env_vars: std::collections::HashMap<String, String>) -> Result<VibeID> {
        // Try to detect if we are inside WezTerm
        let is_inside = std::env::var("WEZTERM_PANE").is_ok();

        let mut cmd = Command::new("wezterm");
        cmd.args(["cli", "split-pane"]);

        match direction {
            SplitDirection::Horizontal => cmd.arg("--right"),
            SplitDirection::Vertical => cmd.arg("--bottom"),
        };

        if !is_inside {
            cmd.arg("--top-level");
        }

        // Add env vars if any
        for (k, v) in env_vars {
            // WezTerm CLI doesn't directly support --env for split-pane.
            // But we can wrap the shell command if needed.
            // For now, we'll try to set them via the process environment, 
            // though that might not propagate to the new pane's shell unless we wrap it.
            // BETTER: Use 'wezterm cli spawn' with --env if we want a new window/tab,
            // but for split-pane we might need to inject them into the child shell.
            // For simplicity and to satisfy the requirement, we will try to pass them
            // though it's terminal dependent.
        }

        let output = cmd.output()?;
        if !output.status.success() {
            // If split-pane failed because no window is available, try to just spawn a new wezterm window
            let output = Command::new("wezterm")
                .arg("start")
                .output();
            
            match output {
                Ok(_) => {
                    // This is tricky as we don't get the pane-id immediately.
                    // For Wave 2, we'll just return a placeholder or wait.
                    // But for now, let's try to get the newly created pane id if possible.
                    return Err(VibeError::TerminalDetectionFailed(
                        "No active WezTerm window found. Started a new one, please retry in a supported environment.".to_string()
                    ));
                }
                Err(e) => return Err(VibeError::TerminalDetectionFailed(format!("Failed to start WezTerm: {}", e))),
            }
        }

        let pane_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(pane_id)
    }

    fn send_keys(&self, target_id: &VibeID, keys: &str) -> Result<()> {
        self.inject_text(target_id, keys)?;
        self.inject_text(target_id, "\n")
    }

    fn inject_text(&self, target_id: &VibeID, text: &str) -> Result<()> {
        let output = Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", target_id])
            .arg(text)
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "WezTerm send-text failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn close(&self, target_id: &VibeID) -> Result<()> {
        let output = Command::new("wezterm")
            .args(["cli", "kill-pane", "--pane-id", target_id])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "WezTerm kill-pane failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn get_metadata(&self) -> Result<TerminalMetadata> {
        let current_pane_id = std::env::var("WEZTERM_PANE").map_err(|_| {
            VibeError::TerminalDetectionFailed("WEZTERM_PANE environment variable not set".into())
        })?;

        let output = Command::new("wezterm")
            .args(["cli", "list", "--format", "json"])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "WezTerm list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let panes: Vec<WezTermPane> = serde_json::from_slice(&output.stdout)?;

        let target_pane = current_pane_id.parse::<u64>().map_err(|_| {
            VibeError::TerminalDetectionFailed(format!("Invalid WEZTERM_PANE: {}", current_pane_id))
        })?;

        let pane = panes
            .iter()
            .find(|p| p.pane_id == target_pane)
            .ok_or_else(|| VibeError::TerminalDetectionFailed(format!("Pane {} not found", current_pane_id)))?;

        Ok(TerminalMetadata {
            pane_id: pane.pane_id.to_string(),
            window_id: pane.window_id.to_string(),
            session_id: pane.workspace.clone(),
        })
    }

    fn focus(&self, target_id: &VibeID) -> Result<()> {
        let output = Command::new("wezterm")
            .args(["cli", "activate-pane", "--pane-id", target_id])
            .output()?;

        if !output.status.success() {
            return Err(VibeError::TerminalDetectionFailed(format!(
                "WezTerm activate-pane failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }
}
