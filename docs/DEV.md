# Mejiro CLI - Developer Guide

## Prerequisites

- Rust (edition 2024)
- `miniserve` for local testing (optional): `cargo install miniserve`

## Project Structure

This is a Rust workspace with 4 modules:

```
mejiro/
├── config/        # Blog configuration (YAML parsing)
├── html/          # HTML generation and markdown parsing
├── mejiro-cli/    # Main CLI application
├── search/        # WASM-based search functionality
```

## Setup

1. Clone the repository
2. Build the project:
   ```bash
   cargo build
   ```

## Running Tests

Run all tests:
```bash
cargo test
```

Run tests for a specific package:
```bash
cargo test --package html
cargo test --package config
cargo test --package mejiro-cli
```

## Development Workflow

### Running the CLI

```bash
# Run from project root
cargo run -- <command>

# Examples:
cargo run -- new          # Create a new blog post
cargo run -- compile      # Compile markdown to HTML
cargo run -- list         # List all posts
```

### Testing Locally

After compiling, serve the static site locally:

```bash
# Compile the blog
cargo run -- compile

# Serve the output directory
miniserve mejiro-cli/public -p 8080

# Open http://localhost:8080 in your browser
```
