use serde::{Deserialize, Serialize};
use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

/// Example authenticated request
#[derive(Serialize, Debug)]
struct SecureRequest {
    message: String,
}

impl ServiceStackRequest for SecureRequest {
    type Response = SecureResponse;

    fn path(&self) -> String {
        "/secure/data".to_string()
    }
}

#[derive(Deserialize, Debug)]
struct SecureResponse {
    data: String,
}

impl ServiceStackResponse for SecureResponse {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new JsonServiceClient
    let mut client = JsonServiceClient::new("https://api.example.com");

    // Set Bearer token for authentication
    client.set_bearer_token("your-bearer-token-here");

    // Create a request
    let request = SecureRequest {
        message: "Secure message".to_string(),
    };

    // Make an authenticated POST request
    println!("Making authenticated request...");
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
