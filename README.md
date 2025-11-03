# ServiceStack Rust Client Library

A Rust client library for making typed API requests to [ServiceStack](https://servicestack.net/) services.

## Features

- 🚀 **Type-safe API requests** - Use Rust DTOs for compile-time safety
- 🔄 **Full HTTP method support** - GET, POST, PUT, DELETE, PATCH
- 🔐 **Bearer token authentication** - Built-in support for authentication
- ⚡ **Async/await** - Built on tokio and reqwest for async operations
- 🎯 **ServiceStack conventions** - Follows ServiceStack's REST API patterns
- 🛠️ **Customizable** - Flexible configuration options

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
servicestack = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use serde::{Deserialize, Serialize};
use servicestack::{JsonServiceClient, ServiceStackRequest, ServiceStackResponse};

// Define your request DTO
#[derive(Serialize)]
struct Hello {
    name: String,
}

// Implement ServiceStackRequest trait
impl ServiceStackRequest for Hello {
    type Response = HelloResponse;
    
    fn path(&self) -> String {
        "/hello".to_string()
    }
}

// Define your response DTO
#[derive(Deserialize, Debug)]
struct HelloResponse {
    result: String,
}

impl ServiceStackResponse for HelloResponse {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = JsonServiceClient::new("https://test.servicestack.net");
    
    // Make a request
    let request = Hello { name: "World".to_string() };
    let response: HelloResponse = client.post(request).await?;
    
    println!("Response: {:?}", response);
    Ok(())
}
```

## Usage

### Creating a Client

```rust
use servicestack::JsonServiceClient;

// Basic client
let client = JsonServiceClient::new("https://api.example.com");

// With custom reqwest client
let http_client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(60))
    .build()?;
let client = JsonServiceClient::with_client("https://api.example.com", http_client);
```

### Making Requests

#### POST Request

```rust
let request = Hello { name: "Rust".to_string() };
let response: HelloResponse = client.post(request).await?;
```

#### GET Request

```rust
let request = GetUser { id: 123 };
let response: UserResponse = client.get(request).await?;
```

#### PUT Request

```rust
let request = UpdateUser { id: 123, name: "John".to_string() };
let response: UpdateResponse = client.put(request).await?;
```

#### DELETE Request

```rust
let request = DeleteUser { id: 123 };
let response: DeleteResponse = client.delete(request).await?;
```

#### Using the request method specified in the DTO

```rust
// The DTO can specify its own HTTP method
impl ServiceStackRequest for MyRequest {
    type Response = MyResponse;
    
    fn path(&self) -> String {
        "/my-endpoint".to_string()
    }
    
    fn method(&self) -> HttpMethod {
        HttpMethod::Put  // Custom method
    }
}

// Use the send method to use the DTO's specified method
let response: MyResponse = client.send(request).await?;
```

### Authentication

```rust
// Set bearer token
let mut client = JsonServiceClient::new("https://api.example.com");
client.set_bearer_token("your-token-here");

// Make authenticated requests
let response = client.post(request).await?;

// Clear token
client.clear_bearer_token();
```

### Error Handling

```rust
use servicestack::ServiceStackError;

match client.post(request).await {
    Ok(response) => println!("Success: {:?}", response),
    Err(ServiceStackError::ApiError { status, message }) => {
        println!("API error {}: {}", status, message);
    }
    Err(ServiceStackError::RequestError(e)) => {
        println!("Request error: {}", e);
    }
    Err(e) => println!("Other error: {}", e),
}
```

### Custom HTTP Methods

You can specify custom HTTP methods by implementing the `method()` function in your request DTO:

```rust
use servicestack::HttpMethod;

impl ServiceStackRequest for CustomRequest {
    type Response = CustomResponse;
    
    fn path(&self) -> String {
        "/custom".to_string()
    }
    
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }
}
```

### Raw Requests

For more control, you can use the raw request method:

```rust
let response: MyResponse = client
    .request("POST", "/custom-endpoint", Some(&request_body))
    .await?;
```

## Examples

See the [examples](examples/) directory for more usage examples:

- [basic_usage.rs](examples/basic_usage.rs) - Simple request/response example
- [authenticated_request.rs](examples/authenticated_request.rs) - Using bearer token authentication
- [custom_method.rs](examples/custom_method.rs) - Custom HTTP methods

Run examples with:

```bash
cargo run --example basic_usage
```

## Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

## API Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

## ServiceStack DTOs

This library works seamlessly with ServiceStack's Add ServiceStack Reference feature. You can generate Rust DTOs from your ServiceStack services and use them directly with this client.

Learn more about ServiceStack's typed client patterns at [docs.servicestack.net](https://docs.servicestack.net/).

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
