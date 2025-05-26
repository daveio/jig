# Hubbit

A command-line tool for managing Git repositories and GitHub release binaries. Because typing full URLs is so last decade.

## Features

- **Smart Repository Cloning**: Clone repos with minimal typing

  - Just the repo name: `hubbit clone myrepo` (uses your configured GitHub username)
  - Owner/repo format: `hubbit clone torvalds/linux`
  - Full URLs: `hubbit clone https://github.com/user/repo`
  - SSH URLs: `hubbit clone git@github.com:user/repo.git`

- **Binary Management**: Download and update GitHub release binaries

  - Download latest: `hubbit binary get cli/cli`
  - Update specific binary: `hubbit binary update user/tool`
  - Update all binaries: `hubbit binary update`

- **Flexible Configuration**: Customize behavior via `~/.config/hubbit/config.yaml`
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Installation

### From Release

Download the latest binary from the [releases page](https://github.com/daveio/hubbit/releases).

### From Source

```bash
go install github.com/daveio/hubbit@latest
```

## Configuration

Create `~/.config/hubbit/config.yaml`:

```yaml
main:
  external_git: false # Use git CLI instead of built-in library

github:
  username: yourusername
  token: your-github-token # Optional, for private repos
  protocol: ssh # or https
  clone_directory: ~/dev
```

## Usage

### Clone a Repository

```bash
# Using your configured username
hubbit clone myrepo

# Explicit owner/repo
hubbit clone daveio/hubbit

# From URL
hubbit clone https://github.com/user/repo
hubbit clone git@github.com:user/repo.git
```

### Manage Binaries

```bash
# Download latest binary for a repo
hubbit binary get owner/repo

# Update a specific binary
hubbit binary update owner/repo

# Update all installed binaries
hubbit binary update
```

### Options

- `-v, --verbose`: Enable verbose output
- `--config`: Use a custom config file

## Binary Detection

Hubbit automatically detects the appropriate binary for your platform:

- Matches OS (linux, darwin/macos, windows)
- Matches architecture (amd64/x86_64, arm64/aarch64)
- Handles archives (.tar.gz, .zip) and plain binaries
- Installs to `~/.local/bin`

## Development

### Prerequisites

- Go 1.24 or later
- Git (for external git mode)

### Building

```bash
go build .
```

### Testing

```bash
go test ./...
```

## License

MIT License - see [LICENSE](LICENSE) file for details.
