# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**belt** is a Go CLI application built with the Kong command parsing library and Charmbracelet's lipgloss for terminal styling. The project demonstrates modern Go CLI patterns with structured command handling, configuration management, and styled output.

## Essential Commands

```bash
# Development environment setup
mise install              # Install Go 1.24.4 and development tools

# Build and run
go build -o belt ./src    # Build the CLI binary
go run ./src/main.go      # Run the application directly

# Code quality
golangci-lint run         # Run Go-specific linters
golangci-lint run --fix   # Run linters with automatic fixes
trunk check               # Run all configured linters (includes golangci-lint)
trunk fmt                 # Auto-format all code

# Testing
go test ./...             # Run all tests (when tests exist)
go test -v ./...          # Run tests with verbose output

# Release
goreleaser build --snapshot --clean  # Test release build locally
```

## Architecture

### Core Structure

- `/src/main.go` - Entry point containing Kong CLI parsing and global flag handling
- `/src/commands/` - Command implementations following Kong's command pattern
- `/src/internal/` - Internal packages for shared functionality
- `/src/config/` - Configuration management with Koanf (env vars, files, defaults)
- `/src/ui/` - Terminal styling definitions using lipgloss

### Command Pattern

The application uses Kong's command parsing with a hierarchical structure:

- Each command is a struct with a `Run(*types.Context) error` method
- Commands can have subcommands (e.g., `format` has `json` subcommand)
- Global flags are defined in the root `CLI` struct and passed via context
- Commands receive a shared `Context` containing config and output writer

### Configuration Layers

Configuration is loaded in priority order using Koanf:

1. Environment variables (BELT\_\* prefix)
2. Config files (belt.yaml, belt.json, ~/.config/belt/config.\*)
3. Default values in code

### Output System

The output system (`internal/output`) supports multiple formats:

- **auto**: Detects based on terminal/pipe context
- **json**: Structured JSON output for piping
- **plain**: No styling, plain text
- **styled**: Terminal colors and formatting with lipgloss

Key pattern: Commands use `ctx.Output` methods for consistent output handling.

### Key Dependencies

- `github.com/alecthomas/kong` - Command-line argument parsing
- `github.com/charmbracelet/lipgloss` - Terminal styling
- `github.com/knadh/koanf/v2` - Configuration management
- Standard library only for core functionality

### Development Patterns

1. **Context Pattern**: All commands receive a `types.Context` with config and output
2. **Error Handling**: Commands return errors up to main for consistent handling
3. **Output Abstraction**: Never write directly to stdout/stderr, use output.Writer
4. **Configuration**: Use Koanf providers for flexible config sources
5. **Styling**: Define styles once in `ui/styles.go`, reuse throughout

## Code Standards

### Linting Rules (enforced by golangci-lint)

- `gofumpt` formatting (stricter than gofmt)
- `thelper`: Test helpers must call t.Helper()
- `tparallel`: Tests should run in parallel with t.Parallel()
- `unconvert`: No unnecessary type conversions
- `unparam`: No unused function parameters
- `wastedassign`: No unused variable assignments

### Git Hooks (via Trunk)

- **Pre-commit**: Automatic code formatting
- **Pre-push**: Full linting and security checks

## Development Workflow

1. **Before starting work**: Run `mise install` to ensure correct Go version
2. **During development**: Use `trunk fmt` to auto-format code
3. **Before committing**: Run `trunk check` to catch any issues
4. **For releases**: GoReleaser handles cross-platform builds automatically

## Testing Approach

When writing tests:

- Place tests in `*_test.go` files alongside the code
- Use table-driven tests for multiple scenarios
- Mock external dependencies using interfaces
- Test command logic separately from CLI parsing
- Use `testify/assert` for clearer test assertions (when added)

## Adding New Commands

1. Create directory under `/src/commands/[name]/`
2. Define command struct with Kong tags for arguments/flags
3. Implement `Run(*types.Context) error` method
4. Add command field to root `CLI` struct in main.go
5. Use context.Output for all output operations

Example command structure:

```go
type Cmd struct {
    // Kong tags for CLI parsing
    File string `arg:"" help:"Input file" type:"path"`
    Format bool `short:"f" help:"Format output"`
}

func (c *Cmd) Run(ctx *types.Context) error {
    // Command implementation
    ctx.Output.PrintSuccess("Done!")
    return nil
}
```

## Security

- DevSkim runs automatically on all pushes and PRs
- Trufflehog scans for secrets in pre-push hooks
- OSV scanner checks for vulnerable dependencies
