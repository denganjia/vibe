use std::env;
use std::path::PathBuf;
use crate::error::{VibeError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType {
    WezTerm,
    Tmux,
}

pub fn detect_current_terminal() -> Result<TerminalType> {
    if env::var("WEZTERM_PANE").is_ok() {
        Ok(TerminalType::WezTerm)
    } else if env::var("TMUX").is_ok() {
        Ok(TerminalType::Tmux)
    } else {
        Err(VibeError::TerminalDetectionFailed(
            "Neither WezTerm nor Tmux detected. Vibe requires a supported terminal environment.".to_string(),
        ))
    }
}

pub fn resolve_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or(VibeError::ConfigDirResolutionFailed)?;
    path.push("vibe");
    Ok(path)
}

pub fn resolve_state_dir() -> Result<PathBuf> {
    // On Unix, data_dir usually points to ~/.local/share
    // On Windows, it points to %LOCALAPPDATA%
    let mut path = dirs::data_dir().ok_or(VibeError::StateDirResolutionFailed)?;
    path.push("vibe");
    Ok(path)
}
