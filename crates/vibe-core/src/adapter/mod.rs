use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowTarget {
    Pane(SplitDirection),
    Tab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalMetadata {
    pub pane_id: String,
    pub window_id: String,
    pub session_id: String,
}

pub type VibeID = String;

pub mod wezterm;
pub use wezterm::WezTermAdapter;

pub mod tmux;
pub use tmux::TmuxAdapter;

pub trait TerminalAdapter: Send + Sync {
    /// Spawn a new terminal context (Pane or Tab) with an optional initial command.
    fn spawn(&self, target: WindowTarget, command: Option<&str>, env_vars: std::collections::HashMap<String, String>) -> Result<VibeID>;

    /// Send keys to the specified pane (simulates typing and hits Enter).
    fn send_keys(&self, target_id: &VibeID, keys: &str) -> Result<()>;

    /// Inject raw text to the specified pane without automatically hitting Enter.
    fn inject_text(&self, target_id: &VibeID, text: &str) -> Result<()>;

    /// Close the specified pane.
    fn close(&self, target_id: &VibeID) -> Result<()>;

    /// Get current terminal metadata.
    fn get_metadata(&self) -> Result<TerminalMetadata>;

    /// Focus the specified pane.
    fn focus(&self, target_id: &VibeID) -> Result<()>;
}
