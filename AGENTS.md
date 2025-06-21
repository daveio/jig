# AGENTS.md

This file provides guidance to AI agents (other than Claude Code) when working with the Belt CLI codebase.

## Project Summary

Belt is a modular CLI toolbelt written in Go that consolidates various utility scripts. Key features:

- Cryptography operations (encryption, key generation)
- DNS operations (lookups, cache flushing, DNSSEC)
- Audio file metadata extraction
- Cross-platform support (Windows, macOS, Linux)

## Quick Start

```bash
# Build
go build -o belt ./src

# Initialize
./belt init

# Example usage
echo "data" | ./belt crypt simple encrypt | ./belt crypt simple decrypt
./belt dns lookup example.com
./belt audio info ~/Music
```

## Technical Stack

- **Language**: Go 1.24+
- **CLI Framework**: Kong (github.com/alecthomas/kong)
- **UI**: Charmbracelet libraries (lipgloss, bubbletea, huh)
- **Config**: Koanf v2 with YAML/JSON support
- **Crypto**: ChaCha20Poly1305 (x/crypto), BLAKE3
- **DNS**: miekg/dns library

## Project Structure

```plaintext
belt/
├── src/
│   ├── main.go                 # Entry point
│   ├── commands/               # Command implementations
│   │   ├── init/              # Config initialization
│   │   ├── audio/             # Audio operations
│   │   ├── crypt/             # Cryptography operations
│   │   ├── dns/               # DNS operations
│   │   ├── domain/            # Domain operations (placeholders)
│   │   └── tls/               # TLS operations (placeholders)
│   ├── internal/              # Shared packages
│   │   ├── crypto/            # Crypto utilities
│   │   ├── platform/          # OS detection/commands
│   │   ├── audio/             # Audio processing
│   │   ├── dns/               # DNS utilities
│   │   └── output/            # Output formatting
│   ├── config/                # Configuration management
│   └── ui/                    # Terminal styling
├── go.mod                     # Go module definition
├── go.sum                     # Dependency checksums
└── mise.toml                  # Development environment

```

## Key Implementation Details

### Command Structure

- Uses Kong's struct-based command definition
- Each command implements `Run(*types.Context) error`
- Hierarchical commands (e.g., `belt crypt random hex`)
- Global flags available to all commands

### Configuration

- YAML config at `~/.config/belt/config.yaml`
- Environment variables override config (BELT\_\* prefix)
- Key settings: encryption key, DNS server

### Encryption

- ChaCha20Poly1305 AEAD encryption
- 32-byte keys encoded in base58
- Block-based processing (1024 bytes)
- BLAKE3 hash for integrity

### Output System

- Multiple formats: styled (default), plain, JSON
- JSON mode activated with `--pipe` flag
- Errors to stderr, data to stdout
- Beautiful terminal output with lipgloss

### Cross-Platform

- Platform detection in `internal/platform`
- OS-specific commands (DNS flush)
- Automatic sudo handling on Unix
- XDG paths for configuration

## Adding New Commands

1. Create directory under `/src/commands/[name]/`
2. Define command struct with Kong tags
3. Implement `Run(*types.Context) error`
4. Add to parent command group or main.go
5. Use `ctx.Output` for all output

Example:

```go
type Cmd struct {
    Input string `arg:"" help:"Input file"`
}

func (c *Cmd) Run(ctx *types.Context) error {
    ctx.Output.PrintSuccess("Done!")
    return nil
}
```

## Testing

Run tests with:

```bash
go test ./...
```

Key areas to test:

- Crypto operations (encrypt/decrypt cycles)
- Platform detection
- Configuration loading
- Command execution

## Common Tasks

### Add a dependency

```bash
go get github.com/some/package
go mod tidy
```

### Format code

```bash
trunk fmt -a
```

### Run linters

```bash
trunk check -a
```

### Build for release

```bash
goreleaser build --snapshot --clean
```

## Design Principles

1. **Modular**: Each command is self-contained
2. **Beautiful**: Terminal output should be visually pleasing
3. **Scriptable**: JSON output for automation
4. **Secure**: Strong crypto, no plaintext secrets
5. **Simple**: Do one thing well

## Security Notes

- Encryption keys are never logged
- Warnings shown for key backup
- Base58 encoding prevents copy/paste errors
- ChaCha20Poly1305 provides authenticated encryption
- BLAKE3 for fast, secure hashing

## Future Commands

Placeholder commands exist for:

- Domain expiry checking
- TLS certificate generation
- Cipher suite listing

These return "not yet implemented" errors.

## Troubleshooting

### Config not found

Run `belt init` to create initial configuration.

### Permission denied

DNS flush and some operations require sudo on Unix systems.

### Audio metadata incorrect

The dhowden/tag library doesn't extract technical metadata (sample rate, bit depth). Default values are used.

## Contributing

1. Follow existing code patterns
2. Use the output system for all user interaction
3. Add tests for new functionality
4. Update documentation
5. Run linters before committing

## Additional Resources

- Kong documentation: <https://github.com/alecthomas/kong>
- Charmbracelet libraries: <https://github.com/charmbracelet>
- Koanf documentation: <https://github.com/knadh/koanf>
- Go crypto documentation: <https://pkg.go.dev/golang.org/x/crypto>
