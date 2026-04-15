use thiserror::Error;

#[derive(Error, Debug)]
pub enum VibeError {
    #[error("Terminal detection failed: {0}")]
    TerminalDetectionFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config directory resolution failed")]
    ConfigDirResolutionFailed,

    #[error("State directory resolution failed")]
    StateDirResolutionFailed,

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Codec error: {0}")]
    Codec(#[from] tokio_util::codec::LinesCodecError),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, VibeError>;
