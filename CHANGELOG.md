# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-11-03

### Added
- Initial release of ServiceStack Rust client library
- `JsonServiceClient` for making typed API requests to ServiceStack services
- Support for all HTTP methods (GET, POST, PUT, DELETE, PATCH)
- `ServiceStackRequest` trait for defining request DTOs
- `ServiceStackResponse` trait for defining response DTOs
- Bearer token authentication support
- Custom error types with detailed error messages
- Async/await support using tokio and reqwest
- Comprehensive test suite with unit and integration tests
- Multiple usage examples demonstrating all features
- Complete API documentation
- CI/CD workflow for automated testing

### Features
- Type-safe API requests using Rust DTOs
- Follows ServiceStack REST API conventions
- Customizable HTTP client configuration
- Flexible error handling
- Support for custom HTTP methods in request DTOs
- Raw request method for advanced use cases
