use serde::{Deserialize, Serialize};
use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

/// Example request DTO
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

    Ok(())
}
