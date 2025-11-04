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

#[derive(Deserialize, Debug)]
struct HelloResponse {
    result: String,
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
