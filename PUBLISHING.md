# Publishing to crates.io

This document describes how to publish the ServiceStack Rust client library to crates.io.

## Prerequisites

1. Create an account on [crates.io](https://crates.io/)
2. Get your API token from [crates.io/me](https://crates.io/me)
3. Configure your token locally:
   ```bash
   cargo login <your-api-token>
   ```

## Pre-publication Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Documentation builds without warnings: `cargo doc --no-deps`
- [ ] Package builds successfully: `cargo package`
- [ ] Version number is correct in `Cargo.toml`
- [ ] `CHANGELOG.md` is updated
- [ ] `README.md` is accurate and up-to-date
- [ ] License file exists and is correct
- [ ] All code is committed and pushed to GitHub

## Package Verification

1. Build and test the package:
   ```bash
   cargo build
   cargo test
   ```

2. Verify the package contents:
   ```bash
   cargo package --list
   ```

3. Build the package to check for issues:
   ```bash
   cargo package
   ```

4. Test the packaged version:
   ```bash
   cargo package
   cd target/package
   cargo test
   ```

## Publishing

Once all checks pass, publish to crates.io:

```bash
cargo publish
```

This will:
1. Package your crate
2. Upload it to crates.io
3. Make it available for download via `cargo install`

## Post-Publication

After publishing:

1. Create a GitHub release:
   - Tag: `v0.1.0`
   - Title: `ServiceStack Rust v0.1.0`
   - Description: Copy from CHANGELOG.md

2. Verify the package appears on crates.io:
   - Visit https://crates.io/crates/servicestack
   - Check that documentation is generated at https://docs.rs/servicestack

3. Test installation:
   ```bash
   cargo new test-project
   cd test-project
   cargo add servicestack
   cargo build
   ```

## Version Updates

For future releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new changes
3. Commit and push changes
4. Run through the pre-publication checklist
5. Publish with `cargo publish`
6. Create a new GitHub release

## Troubleshooting

- **Publishing fails**: Check that all required fields in `Cargo.toml` are filled
- **Documentation fails to build**: Run `cargo doc` locally to see errors
- **Tests fail**: Run `cargo test` to identify and fix failing tests
- **Version conflict**: Ensure version number is incremented from last published version
