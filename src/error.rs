//! Error types for ServiceStack client

use thiserror::Error;

/// Result type alias for ServiceStack operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur when using the ServiceStack client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error with custom message
    #[error("{0}")]
    Message(String),
}

impl Error {
    /// Create a new error with a custom message
    pub fn message<S: Into<String>>(msg: S) -> Self {
        Error::Message(msg.into())
    }
}
