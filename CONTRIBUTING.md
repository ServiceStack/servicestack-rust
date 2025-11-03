# Contributing to ServiceStack Rust

Thank you for your interest in contributing to the ServiceStack Rust client library!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/servicestack-rust.git`
3. Create a branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building the Project

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Running Examples

```bash
cargo run --example basic_usage
cargo run --example complete_example
```

## Code Quality

Before submitting a PR, please ensure:

### 1. Format your code

```bash
cargo fmt
```

### 2. Check for linting issues

```bash
cargo clippy -- -D warnings
```

### 3. All tests pass

```bash
cargo test
```

### 4. Build in release mode

```bash
cargo build --release
```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Add tests for any new functionality
3. Ensure all tests pass
4. Update examples if the API has changed
5. Write clear commit messages
6. Create a pull request with a clear description of the changes

## Code Style

- Follow Rust naming conventions
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and small
- Write tests for new features

## Adding New Features

When adding new features:

1. Add unit tests in the relevant module
2. Add integration tests in the `tests/` directory if needed
3. Update documentation and examples
4. Consider backward compatibility

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system
- Minimal code example that reproduces the issue
- Expected behavior
- Actual behavior

## Questions?

Feel free to open an issue for any questions about contributing.

Thank you for contributing to ServiceStack Rust!
