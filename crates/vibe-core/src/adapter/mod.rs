use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
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
    /// Split the current pane into a new one.
    fn split(&self, direction: SplitDirection, size: Option<u32>) -> Result<VibeID>;

    /// Send keys to the specified pane.
    fn send_keys(&self, target_id: &VibeID, keys: &str) -> Result<()>;

    /// Close the specified pane.
    fn close(&self, target_id: &VibeID) -> Result<()>;

    /// Get current terminal metadata.
    fn get_metadata(&self) -> Result<TerminalMetadata>;

    /// Focus the specified pane.
    fn focus(&self, target_id: &VibeID) -> Result<()>;
}
