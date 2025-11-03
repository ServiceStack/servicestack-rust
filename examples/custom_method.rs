use serde::{Deserialize, Serialize};
use servicestack::{HttpMethod, JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

/// Example request with custom HTTP method
#[derive(Serialize, Debug)]
struct UpdateRequest {
    id: u32,
    name: String,
}

impl ServiceStackRequest for UpdateRequest {
    type Response = UpdateResponse;

    fn path(&self) -> String {
        format!("/users/{}", self.id)
    }

    // Override the default POST method with PUT
    fn method(&self) -> HttpMethod {
        HttpMethod::Put
    }
}

#[derive(Deserialize, Debug)]
struct UpdateResponse {
    success: bool,
    message: String,
}

impl ServiceStackResponse for UpdateResponse {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonServiceClient::new("https://api.example.com");

    let request = UpdateRequest {
        id: 123,
        name: "Updated Name".to_string(),
    };

    println!("Making PUT request with custom method...");
    match client.send(request).await {
        Ok(response) => {
            println!("Success! Response: {:?}", response);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
