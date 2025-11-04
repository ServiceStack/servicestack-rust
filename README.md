# ServiceStack Rust Client Library

A Rust client library for making typed API requests to [ServiceStack](https://servicestack.net/) services.

## Features

- 🚀 **Type-safe API requests** - Use Rust DTOs for compile-time safety
- 🔄 **Full HTTP method support** - GET, POST, PUT, DELETE, PATCH
- 🔐 **Bearer token authentication** - Built-in support for authentication
- ⚡ **Async/await** - Built on tokio and reqwest for async operations
- 🎯 **ServiceStack conventions** - Follows ServiceStack's REST API patterns
- 🛠️ **Customizable** - Flexible configuration options
A Rust client library for ServiceStack services, providing type-safe HTTP communication with async/await support.

[![Crates.io](https://img.shields.io/crates/v/servicestack.svg)](https://crates.io/crates/servicestack)
[![Documentation](https://docs.rs/servicestack/badge.svg)](https://docs.rs/servicestack)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

## Features

- 🚀 **Async/await support** - Built on tokio for efficient async operations
- 📦 **Type-safe** - Leverages Rust's type system with serde for serialization
- 🔧 **Flexible** - Support for all HTTP methods (GET, POST, PUT, DELETE, PATCH)
- 🛡️ **Reliable** - Built on reqwest for robust HTTP communication
- 📖 **Well-documented** - Comprehensive API documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
servicestack = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

```rust
use servicestack::{ServiceStackClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HelloRequest {
    name: String,
}

#[derive(Deserialize)]
struct HelloResponse {
    result: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client
    let client = ServiceStackClient::new("https://example.org");
    
    // Make a POST request
    let request = HelloRequest { 
        name: "World".to_string() 
    };
    let response: HelloResponse = client.post("/hello", &request).await?;
    
    println!("{}", response.result);
    Ok(())
}
```

## Usage Examples

### GET Request

```rust
use servicestack::{ServiceStackClient, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_user(client: &ServiceStackClient, id: u64) -> Result<User> {
    client.get(&format!("/users/{}", id)).await
}
```

### POST Request

```rust
use servicestack::{ServiceStackClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
}

async fn create_user(client: &ServiceStackClient) -> Result<CreateUserResponse> {
    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    client.post("/users", &request).await
}
```

### Custom Client Configuration

```rust
use servicestack::ServiceStackClient;
use reqwest::Client;
use std::time::Duration;

let custom_client = Client::builder()
    .timeout(Duration::from_secs(60))
    .build()
    .unwrap();

let client = ServiceStackClient::with_client(
    "https://api.example.com",
    custom_client
);
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
For detailed API documentation, visit [docs.rs/servicestack](https://docs.rs/servicestack).

## License

This project is licensed under the BSD-3-Clause License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## About ServiceStack

[ServiceStack](https://servicestack.net) is a simple, fast, versatile and highly-productive full-featured Web and Web Services Framework.
