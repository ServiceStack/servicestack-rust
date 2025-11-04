use serde::{de::DeserializeOwned, Serialize};

/// Trait for ServiceStack request DTOs
///
/// Implement this trait on your request types to specify the response type
/// and the API endpoint path.
pub trait ServiceStackRequest: Serialize {
    /// The response type for this request
    type Response: ServiceStackResponse;

    /// Returns the API endpoint path for this request
    ///
    /// Example: "/hello" or "/users/search"
    fn path(&self) -> String;

    /// Returns the HTTP method for this request (defaults to POST)
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }
}

/// Trait for ServiceStack response DTOs
///
/// Implement this trait on your response types to enable deserialization
pub trait ServiceStackResponse: DeserializeOwned {}

/// HTTP methods supported by ServiceStack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
        }
    }
}
