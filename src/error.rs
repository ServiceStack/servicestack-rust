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
}
