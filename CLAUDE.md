# Hubbit Development Guide

## Build & Test Commands

- Build: `cargo build`
- Run: `cargo run -- --name <name> --count <count>`
- Test: `cargo test`
- Test single test: `cargo test <test_name>`
- Format code: `cargo fmt`
- Lint: `cargo clippy`
- Verbose testing: `cargo test -- --nocapture`

## Code Style Guidelines

- **Format**: Use `trunk fmt` for consistent code formatting
- **Lints**: Follow clippy suggestions, use `#[allow(...)]` when necessary
- **Imports**: Group standard library, external crates, then internal modules
- **Naming**: Use snake_case for variables/functions, CamelCase for types
- **Error Handling**: Use Result/Option with descriptive error messages
- **Documentation**: Document public API with rustdoc comments
- **Types**: Prefer strong typing; use newtypes for domain concepts
- **Testing**: Write unit tests for core functionality with descriptive names
