# jig - Documentation for AI Assistants

This document is intended for AI assistants (like Claude) to understand how to use and help develop the `jig` tool.

## Overview

`jig` is a tool for managing development environments. It helps with:

1. Creating new repositories with templates and best practices
2. Updating repositories from templates
3. Configuring AI tools with best practices
4. Bumping dependencies to the latest versions

## Architecture

The codebase is organized into the following modules:

- `cli`: Command-line interface handling
- `commands`: Implementation of the various commands
  - `new`: Creating new repositories
  - `update`: Updating repositories from templates
  - `ai`: Configuring AI tools
  - `bump`: Bumping dependency versions
- `template`: Template handling and language detection
- `git`: Git operations
- `package_manager`: Package manager operations
- `ai`: AI tool configuration
- `config`: Configuration handling
- `utils`: Utility functions

## Template System

Templates are stored in the `templates` directory, organized by language:

```
templates/
  rust/
  python/
  javascript/
  typescript/
  ...
```

Each template directory contains files for that language, with `.tera` extensions for files that should be processed with the Tera template engine.

Template variables:
- `project_name`: The name of the project
- `language`: The programming language

## Implementation Details

### Git Operations

Git operations use the `git2` crate instead of shelling out to git. Key operations include:
- Initializing repositories
- Opening existing repositories
- Committing changes
- Getting the default branch

### Package Manager Detection

The tool auto-detects package managers in repositories:
- Python: pyproject.toml, requirements.txt, setup.py, poetry.lock
- JavaScript/TypeScript: package.json
- Rust: Cargo.toml

### AI Tool Configuration

AI tools are configured by copying configuration files from a baseline repository:
- Claude Desktop: mcp-claude-desktop.json
- Cursor: _cursor directory and mcp-cursor.json
- Zed: .zed directory and mcp-zed.json
- Goose: mcp-goose.yaml

## Extending the Tool

### Adding a New Language

To add support for a new language:
1. Create a new directory in `templates/` for the language
2. Add template files with `.tera` extension for customizable files
3. Update the language detection in `template/language.rs`

### Adding a New Package Manager

To add support for a new package manager:
1. Create a new module in `package_manager/`
2. Implement the `bump_versions` function
3. Add the package manager to the `bump_all_versions` function in `package_manager/mod.rs`

### Adding a New AI Tool

To add support for a new AI tool:
1. Add the tool to the `AiTool` enum in `ai/mod.rs`
2. Implement the `from_str`, `name`, and `config_path` functions
3. Add the tool to the `configure_tool` function

## Common Tasks

### Debugging

The tool uses the `env_logger` crate for logging. Set the `RUST_LOG` environment variable to control logging levels:

```bash
RUST_LOG=debug jig new rust
```

### Testing

Run tests with:

```bash
cargo test
```

### Building

Build the tool with:

```bash
cargo build --release
```

### YAML Handling

The tool uses `yaml-rust2` for YAML parsing instead of the deprecated `serde_yaml`. This is a maintained fork of the original `yaml-rust` that is fully compliant with the YAML 1.2 specification.

For serialization and deserialization with Serde, you would need to use direct mapping between `yaml-rust2::Yaml` and your Serde structures when needed.
