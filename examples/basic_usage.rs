use serde::{Deserialize, Serialize};
use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

/// Example request DTO
//! Basic usage example for the ServiceStack client library
//!
//! This example demonstrates how to use the ServiceStack client
//! to make HTTP requests to a ServiceStack service.

use servicestack::{ServiceStackClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct HelloRequest {
    name: String,
}

/// Implement ServiceStackRequest trait to define the response type and endpoint path
impl ServiceStackRequest for HelloRequest {
    type Response = HelloResponse;

    fn path(&self) -> String {
        "/hello".to_string()
    }
}

/// Example response DTO
#[derive(Deserialize, Debug)]
struct HelloResponse {
    result: String,
}

/// Implement ServiceStackResponse trait to enable deserialization
impl ServiceStackResponse for HelloResponse {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new JsonServiceClient with base URL
    let client = JsonServiceClient::new("https://test.servicestack.net");

    // Create a request
    let request = HelloRequest {
        name: "Rust".to_string(),
    };

    // Make a POST request
    println!("Making request to ServiceStack API...");
    match client.post(request).await {
        Ok(response) => {
            println!("Success! Response: {:?}", response);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new ServiceStack client
    let client = ServiceStackClient::new("https://test.servicestack.net");

    // Example 1: Simple GET request
    println!("Example 1: GET request");
    match client.get::<serde_json::Value>("/hello/World").await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: POST request with JSON body
    println!("\nExample 2: POST request");
    let request = HelloRequest {
        name: "Rust".to_string(),
    };
    
    match client.post::<_, HelloResponse>("/hello", &request).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\nExamples completed!");
    Ok(())
}
