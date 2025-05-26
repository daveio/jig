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
- Clones to organized directory structure: `~/src/github.com/owner/repo`
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
  external_git: true # Use git CLI instead of library

github:
  username: myusername
  token: mytoken # For private repos/API rate limits
  protocol: ssh # or https
  clone_directory: ~/src
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

## Recent Bug Fixes and Improvements

### 2025-05-26

The following bugs were identified and fixed during a comprehensive code review:

1. **Config Error Handling Bug** (`cmd/hubbit/root.go`)

   - **Issue**: Inverted logic was printing errors when config file was not found (normal behavior) instead of actual config reading errors
   - **Fix**: Corrected condition to only log actual configuration errors
   - **Impact**: Eliminates confusing error messages for users without config files

2. **Missing Error Handling in Git Cloner** (`internal/git/cloner.go`)

   - **Issue**: `getDestinationPath()` ignored errors when retrieving user home directory
   - **Fix**: Added proper error handling and propagation for home directory operations
   - **Impact**: Prevents silent failures and provides clear error messages

3. **Missing Error Handling in Binary Manager** (`internal/binary/manager.go`)

   - **Issue**: `NewManager()` constructor ignored home directory retrieval errors
   - **Fix**: Modified constructor to return errors; updated all callers to handle them
   - **Impact**: Fails fast with clear error messages instead of creating invalid managers

4. **URL Parsing Robustness** (`pkg/parser/repository.go`)

   - **Issue**: URL parsing could fail with trailing slashes or consecutive slashes
   - **Fix**: Added filtering of empty path segments to improve parsing reliability
   - **Impact**: Handles malformed URLs more gracefully

5. **Architecture Detection Inconsistency** (`internal/binary/manager.go`)

   - **Issue**: Conflicting architecture mapping logic could cause incorrect binary selection
   - **Fix**: Centralized and corrected architecture alias mapping in `getArchAlias()`
   - **Impact**: Ensures correct binary selection for all supported architectures

6. **Input Validation Enhancement** (`pkg/parser/repository.go`)

   - **Issue**: No validation for empty or whitespace-only repository specifications
   - **Fix**: Added validation to reject invalid input early with clear error messages
   - **Impact**: Better user experience with immediate feedback on invalid input

7. **Test Coverage Enhancement** (`pkg/parser/repository_test.go`)
   - **Issue**: Missing test cases for edge cases and error conditions
   - **Fix**: Added comprehensive test cases for empty and invalid inputs
   - **Impact**: Better test coverage and confidence in error handling

All fixes maintain backward compatibility and follow Go best practices. The codebase now handles edge cases more robustly while preserving existing functionality.

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

## Planned Features and Fixes

See `README.md`.
