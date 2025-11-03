//! # ServiceStack Rust Client Library
//!
//! `servicestack` provides a JsonServiceClient for making typed API requests to ServiceStack services.
//!
//! ## Example
//!
//! ```no_run
//! use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize)]
//! struct Hello {
//!     name: String,
//! }
//!
//! impl ServiceStackRequest for Hello {
//!     type Response = HelloResponse;
//!     fn path(&self) -> String {
//!         "/hello".to_string()
//!     }
//! }
//!
//! #[derive(Deserialize, Debug)]
//! struct HelloResponse {
//!     result: String,
//! }
//!
//! impl ServiceStackResponse for HelloResponse {}
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = JsonServiceClient::new("https://example.org");
//!     let request = Hello { name: "World".to_string() };
//!     let response: HelloResponse = client.post(request).await?;
//!     println!("Response: {:?}", response);
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod traits;

pub use client::JsonServiceClient;
pub use error::{Result, ServiceStackError};
pub use traits::{HttpMethod, ServiceStackRequest, ServiceStackResponse};
