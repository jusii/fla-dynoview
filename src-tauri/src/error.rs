//! A single serializable error type for all Tauri commands.
//!
//! Tauri requires command error types to implement `Serialize`; we serialize to
//! the human-readable `Display` string so the frontend can show it directly.

use fladyno_core::CoreError;

#[derive(Debug)]
pub enum CommandError {
    Io(String),
    Core(String),
    Other(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Io(m) => write!(f, "{m}"),
            CommandError::Core(m) => write!(f, "{m}"),
            CommandError::Other(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for CommandError {}

impl serde::Serialize for CommandError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<CoreError> for CommandError {
    fn from(e: CoreError) -> Self {
        CommandError::Core(e.to_string())
    }
}

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        CommandError::Io(e.to_string())
    }
}
