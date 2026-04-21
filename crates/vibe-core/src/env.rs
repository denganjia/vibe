use std::env;
use std::path::PathBuf;
use crate::error::{VibeError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType {
    WezTerm,
    Tmux,
}

pub fn detect_current_terminal() -> Option<TerminalType> {
    if env::var("WEZTERM_PANE").is_ok() {
        Some(TerminalType::WezTerm)
    } else if env::var("TMUX").is_ok() {
        Some(TerminalType::Tmux)
    } else {
        None
    }
}

pub fn get_terminal_name() -> String {
    if env::var("TERM_PROGRAM").as_deref() == Ok("vscode") {
        "vscode".to_string()
    } else if env::var("WEZTERM_PANE").is_ok() {
        "wezterm".to_string()
    } else if env::var("TMUX").is_ok() {
        "tmux".to_string()
    } else {
        env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
    }
}

pub fn resolve_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or(VibeError::ConfigDirResolutionFailed)?;
    path.push("vibe");
    Ok(path)
}

pub fn resolve_state_dir() -> Result<PathBuf> {
    if let Ok(p) = resolve_project_vibe_dir() {
        return Ok(p.join("state"));
    }
    // Fallback to global state dir
    let mut path = dirs::data_dir().ok_or(VibeError::StateDirResolutionFailed)?;
    path.push("vibe");
    Ok(path)
}

pub fn resolve_project_vibe_dir() -> Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        let vibe_dir = current.join(".vibe");
        if vibe_dir.is_dir() {
            return Ok(vibe_dir);
        }
        if !current.pop() {
            break;
        }
    }
    // Return a default if not found
    Ok(env::current_dir()?.join(".vibe"))
}

pub fn resolve_logs_dir() -> Result<PathBuf> {
    let mut path = resolve_state_dir()?;
    path.push("logs");
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }
    Ok(path)
}

pub fn resolve_plans_dir() -> Result<PathBuf> {
    let mut path = resolve_state_dir()?;
    path.push("plans");
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }
    Ok(path)
}

pub fn resolve_bus_dir() -> Result<PathBuf> {
    let vibe_dir = resolve_project_vibe_dir()?;
    let bus_dir = vibe_dir.join("bus");
    if !bus_dir.exists() {
        std::fs::create_dir_all(&bus_dir)?;
    }
    Ok(bus_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    #[ignore]
    fn test_detect_wezterm() {
        unsafe {
            env::set_var("WEZTERM_PANE", "1");
            env::remove_var("TMUX");
        }
        assert_eq!(detect_current_terminal(), Some(TerminalType::WezTerm));
    }

    #[test]
    #[ignore]
    fn test_detect_tmux() {
        unsafe {
            env::remove_var("WEZTERM_PANE");
            env::set_var("TMUX", "1");
        }
        assert_eq!(detect_current_terminal(), Some(TerminalType::Tmux));
    }

    #[test]
    #[ignore]
    fn test_detect_none() {
        unsafe {
            env::remove_var("WEZTERM_PANE");
            env::remove_var("TMUX");
        }
        assert!(detect_current_terminal().is_none());
    }
}
