use crate::error::{Result, ServiceStackError};
use crate::traits::ServiceStackRequest;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// JsonServiceClient for making typed API requests to ServiceStack services
///
/// This client handles serialization of request DTOs and deserialization of response DTOs,
/// following ServiceStack's conventions for REST API endpoints.
///
/// # Example
///
/// ```no_run
/// use servicestack::JsonServiceClient;
///
/// let client = JsonServiceClient::new("https://api.example.com");
/// ```
#[derive(Clone)]
pub struct JsonServiceClient {
    base_url: String,
    http_client: Client,
    bearer_token: Option<String>,
}

impl JsonServiceClient {
    /// Creates a new JsonServiceClient with the specified base URL
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the ServiceStack API (e.g., "https://api.example.com")
    ///
    /// # Example
    ///
    /// ```
    /// use servicestack::JsonServiceClient;
    ///
    /// let client = JsonServiceClient::new("https://api.example.com");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
            bearer_token: None,
        }
    }

    /// Creates a new JsonServiceClient with a custom reqwest Client
    ///
    /// This allows you to configure the HTTP client with custom settings
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the ServiceStack API
    /// * `http_client` - A configured reqwest Client
    pub fn with_client(base_url: impl Into<String>, http_client: Client) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http_client,
            bearer_token: None,
        }
    }

    /// Sets the Bearer token for authentication
    ///
    /// # Arguments
    ///
    /// * `token` - The Bearer token to use for authentication
    ///
    /// # Example
    ///
    /// ```
    /// use servicestack::JsonServiceClient;
    ///
    /// let mut client = JsonServiceClient::new("https://api.example.com");
    /// client.set_bearer_token("your-token-here");
    /// ```
    pub fn set_bearer_token(&mut self, token: impl Into<String>) {
        self.bearer_token = Some(token.into());
    }

    /// Clears the Bearer token
    pub fn clear_bearer_token(&mut self) {
        self.bearer_token = None;
    }

    /// Makes a GET request to the API
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn get<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        self.send_request(request, "GET").await
    }

    /// Makes a POST request to the API
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn post<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        self.send_request(request, "POST").await
    }

    /// Makes a PUT request to the API
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn put<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        self.send_request(request, "PUT").await
    }

    /// Makes a DELETE request to the API
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn delete<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        self.send_request(request, "DELETE").await
    }

    /// Makes a PATCH request to the API
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn patch<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        self.send_request(request, "PATCH").await
    }

    /// Makes a request using the method specified in the request DTO
    ///
    /// # Arguments
    ///
    /// * `request` - The request DTO implementing ServiceStackRequest
    ///
    /// # Returns
    ///
    /// Returns the response DTO on success
    pub async fn send<T: ServiceStackRequest>(&self, request: T) -> Result<T::Response> {
        let method = request.method().as_str();
        self.send_request(request, method).await
    }

    /// Internal method to send a request
    async fn send_request<T: ServiceStackRequest>(
        &self,
        request: T,
        method: &str,
    ) -> Result<T::Response> {
        let path = request.path();
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = match method.to_uppercase().as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            "PATCH" => self.http_client.patch(&url),
            _ => {
                return Err(ServiceStackError::Other(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        // Add bearer token if set
        if let Some(token) = &self.bearer_token {
            request_builder = request_builder.bearer_auth(token);
        }

        // For methods that support a body, add JSON body
        if method.to_uppercase().as_str() != "GET" {
            request_builder = request_builder.json(&request);
        }

        // Send the request
        let response = request_builder.send().await?;

        // Check status code
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ServiceStackError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Deserialize response
        let response_dto = response.json::<T::Response>().await?;
        Ok(response_dto)
    }

    /// Makes a raw API request with custom serialization
    ///
    /// This is a lower-level method that allows for custom request/response handling
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (GET, POST, etc.)
    /// * `path` - The API endpoint path
    /// * `body` - Optional request body (will be serialized to JSON)
    ///
    /// # Returns
    ///
    /// Returns the deserialized response
    pub async fn request<TRequest, TResponse>(
        &self,
        method: &str,
        path: &str,
        body: Option<&TRequest>,
    ) -> Result<TResponse>
    where
        TRequest: Serialize,
        TResponse: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        let mut request_builder = match method.to_uppercase().as_str() {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            "PATCH" => self.http_client.patch(&url),
            _ => {
                return Err(ServiceStackError::Other(format!(
                    "Unsupported HTTP method: {}",
                    method
                )))
            }
        };

        // Add bearer token if set
        if let Some(token) = &self.bearer_token {
            request_builder = request_builder.bearer_auth(token);
        }

        // Add body if provided
        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        // Send the request
        let response = request_builder.send().await?;

        // Check status code
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ServiceStackError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Deserialize response
        let response_dto = response.json::<TResponse>().await?;
        Ok(response_dto)
    }

    /// Returns the base URL of the client
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns a reference to the underlying HTTP client
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ServiceStackResponse;
    use serde::Deserialize;

    #[derive(Serialize)]
    struct TestRequest {
        name: String,
    }

    impl ServiceStackRequest for TestRequest {
        type Response = TestResponse;

        fn path(&self) -> String {
            "/test".to_string()
        }
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestResponse {
        result: String,
    }

    impl ServiceStackResponse for TestResponse {}

    #[test]
    fn test_client_creation() {
        let client = JsonServiceClient::new("https://api.example.com");
        assert_eq!(client.base_url(), "https://api.example.com");
    }

    #[test]
    fn test_client_creation_with_trailing_slash() {
        let client = JsonServiceClient::new("https://api.example.com/");
        assert_eq!(client.base_url(), "https://api.example.com");
    }

    #[test]
    fn test_bearer_token() {
        let mut client = JsonServiceClient::new("https://api.example.com");
        assert!(client.bearer_token.is_none());

        client.set_bearer_token("test-token");
        assert_eq!(client.bearer_token.as_deref(), Some("test-token"));

        client.clear_bearer_token();
        assert!(client.bearer_token.is_none());
    }
}
