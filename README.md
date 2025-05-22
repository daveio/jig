# jig

A tool to manage various aspects of a development environment.

## Features

- Auto-detect languages, package managers, and configurations
- Create new repositories with best practices and templates
- Update repositories from templates
- Configure AI tools with best practices
- Bump dependencies to latest versions
- Update GitHub Actions workflows

## Installation

### From Source

```bash
git clone https://github.com/daveio/jig.git
cd jig
cargo install --path .
```

## Dependency Information

`jig` uses the following key dependencies:

- `clap`: 4.5.38 - Command line argument parsing
- `tera`: 1.20.0 - Template rendering
- `git2`: 0.20.2 - Git operations
- `yaml-rust2`: 0.10.2 - YAML parsing
- `anyhow` and `thiserror`: 1.0.98/2.0.12 - Error handling

## Usage

### Creating a New Repository

```bash
jig new rust
```

This command will:

1. Initialize a Git repository
2. Create a basic project structure for the specified language
3. Set up CI/CD with GitHub Actions
4. Commit the changes

Supported languages:

- Rust
- Python
- JavaScript
- TypeScript
- Go
- Java
- Ruby
- PHP
- C#
- C++
- C
- Shell

### Updating a Repository

```bash
jig update
```

This command will update the current repository from the template. You can also specify a different repository:

```bash
jig update /path/to/repo
```

### Setting Up AI Support

```bash
jig ai
```

This will configure all supported AI tools. You can also specify a single tool:

```bash
jig ai cursor
```

Supported AI tools:

- Claude Desktop
- Cursor
- Zed
- Goose

### Bumping Versions

```bash
jig bump
```

This command will:

1. Detect package managers in the repository
2. Update dependencies to the latest versions
3. Migrate to modern package managers if needed (e.g., Python projects to `uv`)
4. Update GitHub Actions workflows to the latest versions
5. Commit the changes

## Dry Run Mode

All commands support the `--dry-run` flag to show what would be changed without actually making changes:

```bash
jig bump --dry-run
```

## License

MIT
