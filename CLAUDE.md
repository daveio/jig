# Hubbit - Project Context for AI Assistants

## Overview

Hubbit is a command-line tool for managing Git repositories and GitHub release binaries. It's written in Go and uses Cobra for CLI functionality.

## Core Functionality

### 1. Repository Cloning (`hubbit clone`)

- Parses various repository formats:
  - Simple name: `hubbit clone myrepo` (uses configured GitHub username)
  - Owner/repo: `hubbit clone torvalds/linux`
  - Full URLs: `hubbit clone https://github.com/user/repo`
  - SSH URLs: `hubbit clone git@github.com:user/repo.git`
- Clones to organized directory structure: `~/dev/github.com/owner/repo`
- Supports both HTTPS and SSH protocols
- Can use either go-git library or external git CLI

### 2. Binary Management (`hubbit binary`)

- Downloads GitHub release binaries: `hubbit binary get owner/repo`
- Updates installed binaries: `hubbit binary update [repo]`
- Tracks installed binaries in `~/.config/hubbit/binaries.yaml`
- Automatically detects platform/architecture
- Handles archives (.tar.gz, .zip) and plain binaries
- Installs to `~/.local/bin`

## Project Structure

```plaintext
.
├── cmd/hubbit/          # CLI commands (Cobra)
│   ├── root.go         # Root command and config initialization
│   ├── clone.go        # Clone command implementation
│   └── binary.go       # Binary management commands
├── internal/           # Private packages
│   ├── git/           # Git operations
│   │   └── cloner.go  # Repository cloning logic
│   └── binary/        # Binary management
│       ├── manager.go # Binary download/update logic
│       └── types.go   # Data structures
├── pkg/               # Public packages
│   └── parser/        # Repository parsing
│       ├── repository.go      # Parse repo specifications
│       └── repository_test.go # Tests
├── .github/workflows/  # CI/CD
│   ├── build.yaml     # Build and test
│   ├── check.yaml     # Linting and security
│   └── release.yaml   # Release automation
└── Configuration files
    ├── .goreleaser.yaml # Release configuration
    ├── .golangci.yml    # Linting rules
    └── Makefile         # Development tasks
```

## Key Design Decisions

1. **Dual Git Support**: Can use either go-git library (default) or external git CLI (configurable)
2. **Smart Parsing**: Repository parser handles multiple input formats intelligently
3. **Platform Detection**: Binary manager automatically selects appropriate assets for OS/arch
4. **Configuration**: Uses Viper for flexible config management (YAML, env vars)
5. **Error Handling**: Comprehensive error messages with context

## Configuration

User configuration at `~/.config/hubbit/config.yaml`:

```yaml
main:
  external_git: false # Use git CLI instead of library

github:
  username: myusername
  token: mytoken # For private repos/API rate limits
  protocol: ssh # or https
  clone_directory: ~/dev
```

## Dependencies

- **cobra**: CLI framework
- **viper**: Configuration management
- **go-git**: Git operations (when not using external git)
- **go-github**: GitHub API client
- **yaml.v3**: YAML parsing

## Development

- Run tests: `make test` or `go test ./...`
- Build: `make build` or `go build .`
- Lint: `make lint` (requires golangci-lint)
- Format: `make fmt`

## Testing

- Unit tests for parser and binary manager
- Tests use table-driven approach
- No integration tests currently (would need mocking GitHub API)

## Future Enhancements (from PROMPT.md)

- Multiple GitHub account support
- Other Git platforms (GitLab, Bitbucket)
- Advanced repository organization
- Binary version management
- Package manager integration
- Repository statistics

## Common Tasks

### Adding a new command

1. Create new file in `cmd/hubbit/`
2. Define command with `cobra.Command`
3. Add to parent command in `init()`
4. Implement command logic in `RunE` function

### Adding new repository format support

1. Update regex patterns in `pkg/parser/repository.go`
2. Add parsing logic in `ParseRepository()`
3. Add tests in `repository_test.go`

### Extending binary detection

1. Update `findBestAsset()` in `internal/binary/manager.go`
2. Add platform/arch aliases if needed
3. Update archive extraction if new format

## Important Notes

- Binary installation requires `~/.local/bin` in PATH
- SSH cloning requires SSH keys configured
- GitHub token recommended for API rate limits
- Verbose mode (`-v`) provides detailed output
- All paths in config can use `~` for home directory
