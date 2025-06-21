# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**belt** is a Go CLI application built with the Bubble Tea TUI framework. The project uses the Charmbracelet ecosystem for terminal UI development.

## Essential Commands

```bash
# Development environment setup
mise install              # Install Go 1.24.4 and development tools

# Build and run
go build -o belt ./src    # Build the CLI binary
go run ./src/main.go      # Run the application directly

# Code quality
golangci-lint run         # Run Go-specific linters
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

- `/src/main.go` - Entry point containing the Bubble Tea application
- The app follows Bubble Tea's Model-Update-View pattern:
  - `model` struct holds application state
  - `Update()` handles messages and state changes
  - `View()` renders the terminal UI
  - `Init()` returns initial commands

### Key Dependencies

- `github.com/charmbracelet/bubbletea` - Main TUI framework
- `github.com/charmbracelet/bubbles` - Pre-built UI components (spinner, etc.)
- `github.com/charmbracelet/lipgloss` - Terminal styling

### Development Patterns

1. **Bubble Tea Message Flow**: All state changes happen through messages in Update()
2. **Styling**: Use lipgloss for consistent terminal styling
3. **Components**: Leverage bubbles for common UI elements before building custom ones

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
- Use `t.Helper()` in test helper functions
- Add `t.Parallel()` to tests that can run concurrently
- Focus on testing business logic and state transitions in Bubble Tea models

## Security

- DevSkim runs automatically on all pushes and PRs
- Trufflehog scans for secrets in pre-push hooks
- OSV scanner checks for vulnerable dependencies
