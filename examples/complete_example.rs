/// Complete example demonstrating all features of the ServiceStack Rust client
use serde::{Deserialize, Serialize};
use servicestack::{HttpMethod, JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

// Example 1: Basic request/response
#[derive(Serialize, Debug)]
struct HelloRequest {
    name: String,
}

impl ServiceStackRequest for HelloRequest {
    type Response = HelloResponse;

    fn path(&self) -> String {
        "/hello".to_string()
    }
}

#[derive(Deserialize, Debug)]
struct HelloResponse {
    result: String,
}

impl ServiceStackResponse for HelloResponse {}

// Example 2: GET request with query parameters
#[derive(Serialize, Debug)]
struct GetUserRequest {
    id: u32,
}

impl ServiceStackRequest for GetUserRequest {
    type Response = UserResponse;

    fn path(&self) -> String {
        format!("/users/{}", self.id)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}

#[derive(Deserialize, Debug)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

impl ServiceStackResponse for UserResponse {}

// Example 3: PUT request for updates
#[derive(Serialize, Debug)]
struct UpdateUserRequest {
    id: u32,
    name: String,
    email: String,
}

impl ServiceStackRequest for UpdateUserRequest {
    type Response = UpdateResponse;

    fn path(&self) -> String {
        format!("/users/{}", self.id)
    }

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

// Example 4: DELETE request
#[derive(Serialize, Debug)]
struct DeleteUserRequest {
    id: u32,
}

impl ServiceStackRequest for DeleteUserRequest {
    type Response = DeleteResponse;

    fn path(&self) -> String {
        format!("/users/{}", self.id)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }
}

#[derive(Deserialize, Debug)]
struct DeleteResponse {
    success: bool,
}

impl ServiceStackResponse for DeleteResponse {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ServiceStack Rust Client - Complete Example ===\n");

    // Create a client instance
    let base_url = "https://test.servicestack.net";
    let mut client = JsonServiceClient::new(base_url);

    println!("1. Basic POST request:");
    println!("   Making POST request to {}/hello", base_url);
    let hello_request = HelloRequest {
        name: "Rust Developer".to_string(),
    };

    match client.post(hello_request).await {
        Ok(response) => {
            println!("   ✓ Success: {:?}\n", response);
        }
        Err(e) => {
            println!("   ✗ Error: {}\n", e);
        }
    }

    println!("2. GET request:");
    println!("   This would make a GET request to retrieve a user");
    let get_request = GetUserRequest { id: 1 };
    println!("   Request path: {}", get_request.path());
    println!("   (Skipping actual request to avoid errors on demo endpoint)\n");

    println!("3. PUT request with authentication:");
    println!("   Setting Bearer token for authentication");
    client.set_bearer_token("demo-token-12345");

    let update_request = UpdateUserRequest {
        id: 1,
        name: "Updated Name".to_string(),
        email: "updated@example.com".to_string(),
    };
    println!("   Request path: {}", update_request.path());
    println!("   (Skipping actual request to avoid errors on demo endpoint)\n");

    println!("4. DELETE request:");
    let delete_request = DeleteUserRequest { id: 1 };
    println!("   Request path: {}", delete_request.path());
    println!("   Method: {:?}", delete_request.method());
    println!("   (Skipping actual request to avoid errors on demo endpoint)\n");

    println!("5. Using the send() method:");
    println!("   The send() method uses the HTTP method specified in the request DTO");
    println!("   This allows each DTO to define its own preferred HTTP method\n");

    println!("6. Error handling:");
    println!("   Making a request to a non-existent endpoint");
    let bad_request = HelloRequest {
        name: "Test".to_string(),
    };

    // Try to make a request to a bad endpoint using raw request method
    match client
        .request::<HelloRequest, HelloResponse>("POST", "/nonexistent", Some(&bad_request))
        .await
    {
        Ok(_) => println!("   ✓ Unexpected success\n"),
        Err(e) => println!("   ✓ Expected error: {}\n", e),
    }

    println!("7. Client configuration:");
    println!("   Base URL: {}", client.base_url());
    println!("   Authentication: Bearer token configured");
    println!("   HTTP Client: reqwest (with 30s timeout by default)\n");

    println!("=== Example completed successfully! ===");

    Ok(())
}
