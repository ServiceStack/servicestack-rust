//! # ServiceStack Rust Client Library
//!
//! A Rust client library for ServiceStack services.
//!
//! ## Features
//!
//! - Async/await support with tokio
//! - JSON serialization with serde
//! - Type-safe request/response handling
//! - Built on reqwest for reliable HTTP communication
//!
//! ## Example
//!
//! ```no_run
//! use servicestack::{ServiceStackClient, Result};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize)]
//! struct HelloRequest {
//!     name: String,
//! }
//!
//! #[derive(Deserialize)]
//! struct HelloResponse {
//!     result: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = ServiceStackClient::new("https://example.org");
//!     let request = HelloRequest { name: "World".to_string() };
//!     let response: HelloResponse = client.post("/hello", &request).await?;
//!     println!("{}", response.result);
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod error;
pub use error::{Error, Result};

/// ServiceStack HTTP client for making requests to ServiceStack services
#[derive(Debug, Clone)]
pub struct ServiceStackClient {
    base_url: String,
    client: Client,
}

impl ServiceStackClient {
    /// Create a new ServiceStack client with the given base URL
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the ServiceStack service (e.g., <https://api.example.com>)
    ///
    /// # Example
    ///
    /// ```
    /// use servicestack::ServiceStackClient;
    ///
    /// let client = ServiceStackClient::new("https://api.example.com");
    /// ```
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.into(),
            client,
        }
    }

    /// Build a full URL from the base URL and path
    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Send a request and deserialize the JSON response
    async fn send_json<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = response.error_for_status()?;
        let result = response.json::<T>().await?;
        Ok(result)
    }

    /// Create a new ServiceStack client with a custom reqwest Client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the ServiceStack service
    /// * `client` - A pre-configured reqwest Client
    ///
    /// # Example
    ///
    /// ```
    /// use servicestack::ServiceStackClient;
    /// use reqwest::Client;
    ///
    /// let custom_client = Client::builder()
    ///     .timeout(std::time::Duration::from_secs(60))
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = ServiceStackClient::with_client("https://api.example.com", custom_client);
    /// ```
    pub fn with_client<S: Into<String>>(base_url: S, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    /// Make a GET request to the specified path
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path (e.g., "/hello/World")
    ///
    /// # Returns
    ///
    /// A deserialized response of type `T`
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(path);
        let response = self.client.get(&url).send().await?;
        self.send_json(response).await
    }

    /// Make a POST request with a JSON body
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path (e.g., "/hello")
    /// * `body` - The request body to be serialized as JSON
    ///
    /// # Returns
    ///
    /// A deserialized response of type `T`
    pub async fn post<S, T>(&self, path: &str, body: &S) -> Result<T>
    where
        S: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(path);
        let response = self.client.post(&url).json(body).send().await?;
        self.send_json(response).await
    }

    /// Make a PUT request with a JSON body
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - The request body to be serialized as JSON
    ///
    /// # Returns
    ///
    /// A deserialized response of type `T`
    pub async fn put<S, T>(&self, path: &str, body: &S) -> Result<T>
    where
        S: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(path);
        let response = self.client.put(&url).json(body).send().await?;
        self.send_json(response).await
    }

    /// Make a DELETE request
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    ///
    /// # Returns
    ///
    /// A deserialized response of type `T`
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(path);
        let response = self.client.delete(&url).send().await?;
        self.send_json(response).await
    }

    /// Make a PATCH request with a JSON body
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - The request body to be serialized as JSON
    ///
    /// # Returns
    ///
    /// A deserialized response of type `T`
    pub async fn patch<S, T>(&self, path: &str, body: &S) -> Result<T>
    where
        S: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = self.build_url(path);
        let response = self.client.patch(&url).json(body).send().await?;
        self.send_json(response).await
    }

    /// Get the base URL of this client
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ServiceStackClient::new("https://api.example.com");
        assert_eq!(client.base_url(), "https://api.example.com");
    }

    #[test]
    fn test_client_with_custom_client() {
        let custom_client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        
        let client = ServiceStackClient::with_client("https://api.example.com", custom_client);
        assert_eq!(client.base_url(), "https://api.example.com");
    }
}
