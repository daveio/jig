# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**belt** is a modular CLI toolbelt that consolidates various utility scripts into a single command-line application. It provides utilities for cryptography operations (encryption, key generation), DNS operations (lookups, cache flushing, DNSSEC validation), audio file metadata inspection, and more.

## Essential Commands

```bash
# Development environment setup
mise install              # Install Go 1.24.4 and development tools

# Build and run
go build -o belt ./src    # Build the CLI binary
go run ./src/main.go      # Run the application directly

# Code quality
mise lint                 # Run Go-specific linters
mise lint:fix             # Run linters with automatic fixes
trunk check -a            # Run all configured linters (includes golangci-lint)
trunk fmt -a              # Auto-format all code

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
  - `init/` - Configuration initialization
  - `audio/info/` - Audio metadata extraction
  - `crypt/` - Cryptography operations (random, simple, wireguard)
  - `dns/` - DNS operations (flush, lookup, sec)
  - `domain/` - Domain operations (placeholders)
  - `tls/` - TLS operations (placeholders)
- `/src/internal/` - Internal packages for shared functionality
  - `crypto/` - ChaCha20Poly1305, BLAKE3, base58, X25519 utilities
  - `platform/` - OS detection, command execution, sudo handling
  - `audio/` - Audio file detection and metadata extraction
  - `dns/` - DNS resolver, DNSSEC validation
  - `output/` - Formatted output with JSON/plain/styled modes
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
2. Config files (~/.config/belt/config.yaml via XDG)
3. Default values in code

Key configuration:

- `crypt.key` - Base58-encoded 32-byte encryption key
- `crypt.env` - Environment variable name for key override
- `dns.server` - Default DNS server (1.1.1.1)
- `dns.root` - Use root servers for queries

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
- `github.com/charmbracelet/bubbletea` - TUI framework
- `github.com/charmbracelet/huh` - Interactive forms
- `github.com/knadh/koanf/v2` - Configuration management
- `github.com/adrg/xdg` - XDG BaseDirectory paths
- `github.com/btcsuite/btcutil` - Base58 encoding
- `golang.org/x/crypto` - ChaCha20Poly1305, X25519
- `lukechampine.com/blake3` - BLAKE3 hashing
- `github.com/miekg/dns` - DNS operations
- `github.com/dhowden/tag` - Audio metadata extraction

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

## Implemented Commands

### Initialization

- `belt init` - Create config file with new encryption key

### Cryptography

- `belt crypt random hex [length]` - Generate random hex strings
- `belt crypt random pw [length]` - Generate secure passwords
- `belt crypt simple encrypt` - Encrypt stdin with ChaCha20Poly1305
- `belt crypt simple decrypt` - Decrypt stdin
- `belt crypt simple key` - Display or generate encryption key
- `belt crypt wire-guard` - Generate WireGuard X25519 keypair

### Audio

- `belt audio info <path>` - Extract metadata (sample rate, bit depth, size)

### DNS

- `belt dns flush` - Platform-specific DNS cache flushing
- `belt dns lookup <domain> [type]` - DNS queries with custom server support
- `belt dns sec <domain>` - DNSSEC validation

### Placeholders

- `belt domain expiry/ns` - Not implemented
- `belt tls cert req/selfsign` - Not implemented
- `belt tls ciphers` - Not implemented

## Implementation Details

### Encryption Scheme

- Algorithm: ChaCha20Poly1305 AEAD
- Key: 32 bytes, base58-encoded for storage
- Nonce: 12 bytes per block
- Hash: BLAKE3 64-byte hash as associated data
- Block size: 1024 bytes for streaming
- Output format: base58(nonce + ciphertext + hash) with "\n" delimiter

### Audio Metadata

- Uses dhowden/tag library for format detection
- Returns default values for sample rate/bit depth (library limitation)
- Format: `filepath:samplerate:bitdepth:filesize`

### Platform Support

- DNS flush: Windows (ipconfig), macOS (dscacheutil + mDNSResponder), Linux (systemd-resolve/nscd)
- Sudo handling: Automatic on Unix systems
- Cross-platform path handling via XDG

## Security

- DevSkim runs automatically on all pushes and PRs
- Trufflehog scans for secrets in pre-push hooks
- OSV scanner checks for vulnerable dependencies
- Encryption keys never logged or displayed without user action
- Warning boxes written to stderr to avoid data contamination
