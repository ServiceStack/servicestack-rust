use thiserror::Error;

/// Result type for ServiceStack operations
pub type Result<T> = std::result::Result<T, ServiceStackError>;

/// Error types that can occur when using ServiceStack client
#[derive(Error, Debug)]
pub enum ServiceStackError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
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
