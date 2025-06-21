# Belt CLI Go Implementation Plan

## Project Overview

Belt is a modular CLI toolbelt collecting my various utility scripts, providing utilities for:

- Cryptography operations (encryption, key generation)
- DNS operations (lookups, cache flushing, DNSSEC validation)
- TLS certificate management (planned)
- Domain operations (planned)
- Audio file metadata inspection
- Ability to easily expand to more features

### Architecture Principles

1. **Modular Design**: Each command is self-contained in its own directory
2. **Shared Configuration**: YAML-based config with environment variable override support
3. **Hierarchical Commands**: `belt [command] [subcommand] [...] {args}` structure
4. **Cross-platform Support**: Windows, macOS, and Linux compatibility

## Core Libraries

### `github.com/alecthomas/kong` - Command Line Argument Parsing

Kong is a lightweight, declarative command-line parser that uses struct tags to define CLI behavior.

It handles:

- Command-line flags and arguments
- Sub-commands and command hierarchies
- Help text generation
- Validation of user input

Kong has gained popularity as a cleaner, more modern alternative to Cobra for CLI argument parsing.

### `github.com/knadh/koanf/v2` - Configuration Management

Koanf is a configuration library.

We need to use `v2` as `v1` is six years old.

`koanf` is a library for reading configuration from different sources in different formats in Go applications. It is a cleaner, lighter alternative to spf13/viper with better abstractions and extensibility and far fewer dependencies.

`koanf` v2 has modules (Providers) for reading configuration from a variety of sources such as files, command line flags, environment variables, Vault, and S3 and for parsing (Parsers) formats such as JSON, YAML, TOML, Hashicorp HCL. It is easy to plug in custom parsers and providers.

All external dependencies in providers and parsers are detached from the core and can be installed separately as necessary.

Concepts

`koanf.Provider` is a generic interface that provides configuration, for example, from files, environment variables, HTTP sources, or anywhere. The configuration can either be raw bytes that a parser can parse, or it can be a nested map[string]interface{} that can be directly loaded.

`koanf.Parser` is a generic interface that takes raw bytes, parses, and returns a nested map[string]interface{}. For example, JSON and YAML parsers.

Once loaded into koanf, configuration are values queried by a delimited key path syntax. eg: app.server.port. Any delimiter can be chosen.

Configuration from multiple sources can be loaded and merged into a `koanf` instance, for example, load from a file first and override certain values with flags from the command line.

With these two interface implementations, `koanf` can obtain configuration in any format from any source, parse it, and make it available to an application.

#### Charmbracelet Libraries - Terminal UI

The `charmbracelet` ecosystem provides libraries for building rich terminal user interfaces:

- <https://github.com/charmbracelet/bubbletea>: A framework for building terminal applications based on The Elm Architecture
- <https://github.com/charmbracelet/bubbles>: A collection of UI components for Bubble Tea
- <https://github.com/charmbracelet/lipgloss>: A styling library for terminal applications
- <https://github.com/charmbracelet/huh>: A library for interactive forms and prompts

We want pretty output at all times, ideally abstracted to shared helpers to make implementation easier in each command. The `charmbracelet` libraries provide this, and you should ALWAYS feel free to pull in more libraries.

## Configuration System

### File Location and Structure

**Path**: `~/.config/belt/config.yaml` (XDG BaseDirectory compliant)

```yaml
crypt:
  env: BELT_CRYPT_KEY # Environment variable name for encryption key
  key: <base58-encoded-key> # Default encryption key (32 bytes, base58 encoded)
  warned: false # Whether user has been warned about key backup

dns:
  server: 1.1.1.1 # Default DNS server for lookups
  root: false # Whether to use root servers by default
```

### Configuration Behavior

Use `github.com/knadh/koanf/v2`

Treat things set to `null` in YAML as if they don't exist.

1. Tell user about `belt init` if directory or file does not exist, or file is invalid
1. Environment variable (specified in `crypt.env`) takes precedence over `crypt.key`
1. Missing sections default to safe values
1. Graceful failure with helpful error messages
1. Use `github.com/knadh/koanf/v2` library for configuration management

## Shared Arguments

Arguments which are available in all commands (but may be ignored):

- `-a` / `--all` : All: show all information or operate on all arguments.
- `-f` / `--force` : Force: force overwrite or other destructive operation.
- `-h` / `--help` : Help: Give usage message and exit.
- `-i` / `--input` : Read input from filename.
- `-l` / `--list` : List: list files or arguments without taking other action.
- `-o` / `--output` : Write output to filename.
- `-p` / `--pipe` : Output structured data as JSON for use in a pipe.
- `-q` / `--quiet` : Quiet: less output to stdout.
- `-r` / `--recursive` : Recursive: Operate recursively (down directory tree).
- `-s` / `--silent` : Silent: No output to stdout.
- `-v` / `--verbose` : Verbose: output additional information to stdout or stderr.
- `-V` / `--version` : Version: Show program version and exit.
- `-z` / `--compress` : Compress: apply zstd compression.

## Command Implementation Details

### 1. `belt init` - Configuration Initialization

**Purpose**: Initialize or reinitialize configuration file

**Location**: `src/commands/init/main.go`

**Arguments**: None

**Options**:

- `-w, --write`: Overwrite existing config without confirmation

**Behavior**:

1. Check if config file exists at `~/.config/belt/config.yaml`
2. If exists and no `--overwrite` flag:
   - Prompt user for confirmation (using Charmbracelet library)
3. Generate new 32-byte encryption key using crypto/rand
4. Encode key using base58 library, with Bitcoin alphabet
5. Create directory structure if needed
6. Write default YAML configuration
7. Exit with success message

**Dependencies**:

- `adrg/xdg` for XDG path resolution
- `gopkg.in/yaml.v3` for YAML writing
- `btcsuite/btcutil/base58` for base58 encoding

### 2. `belt audio` - Audio File Operations

#### 2.1 `belt audio info <path>` - Audio Metadata Extraction

**Purpose**: Extract and display audio file metadata

**Location**: `src/commands/audio/info/main.go`

**Arguments**:

- `path`: File or directory path to analyze

**Behavior**:

1. If path is directory, recursively walk tree
2. Identify audio files by extension (case-insensitive):
   - `.flac`, `.mp3`, `.m4a`
3. For each audio file, extract:
   - Sample rate
   - Bits per sample (MP3 defaults to 16)
   - File size in bytes
4. Output format: `{filepath}:{sample_rate}:{bits_per_sample}:{file_size}`
5. One line per audio file to stdout

**Dependencies**:

- `dhowden/tag` or similar for metadata extraction

### 3. `belt crypt` - Cryptography Operations

#### 3.1 `belt crypt random` - Random Data Generation

##### 3.1.1 `belt crypt random hex [length]` - Hex String Generation

**Location**: `src/commands/crypt/random/hex/main.go`

**Arguments**:

- `length`: Number of bytes (default: 16)

**Behavior**:

1. Generate cryptographically secure random bytes using crypto/rand
2. Convert to hex string (length \* 2 characters)
3. Output to stdout

##### 3.1.2 `belt crypt random pw [length]` - Password Generation

**Location**: `src/commands/crypt/random/pw/main.go`

**Arguments**:

- `length`: Password length (default: 16)

**Behavior**:

1. Character set: letters + digits + `-_.@#$%&*+=:`
2. First character: always a digit
3. Last character: always punctuation
4. Middle characters: random from full alphabet
5. Ensure at least one digit and one punctuation
6. Output to stdout

#### 3.2 `belt crypt simple` - Simple Encryption/Decryption

##### 3.2.1 `belt crypt simple encrypt` - Encrypt Data

**Location**: `src/commands/crypt/simple/encrypt/main.go`

**Input**: Plaintext from stdin (binary mode)
**Output**: Base58-encoded ciphertext to stdout

**Behavior**:

1. Show key backup warning (unless `crypt.warned: true` in config)
2. Read encryption key from env var or config
3. Read data in blocks of up to 1024 bytes from `stdin`
4. For each 1024-byte block of plaintext, or whole plaintext if less than 1024 bytes:
5. Generate 12-byte random nonce.
6. Compute BLAKE3 64-byte hash of plaintext.
7. Encrypt using ChaCha20Poly1305 AEAD with hash as associated data.
8. Construct output block: nonce(12) + ciphertext(variable length) + hash(64).
9. Encode block using base58 (Bitcoin alphabet).
10. Write block to `stdout` followed by `||||` delimiter.
11. Move to next block.

This construction allows us to read and write in blocks. This saves having to load the entire input into memory at once. If there are any issues with this approach, adjust it to solve them and document the problem and the fix in the `README.md`.

##### 3.2.2 `belt crypt simple decrypt` - Decrypt Data

**Location**: `src/commands/crypt/simple/decrypt/main.go`

**Input**: Ciphertext from `stdin` (`||||`-delimited blocks)
**Output**: Decrypted plaintext to stdout (binary mode)

**Behavior**:

1. Show key backup warning (unless `crypt.warned: true`)
2. Read encryption key from env var or config
3. Reverse encryption process
4. If any blocks have a hash mismatch: error "Invalid ciphertext"
5. Output raw binary data to stdout

##### 3.2.3 `belt crypt simple key` - Generate New Key

**Location**: `src/commands/crypt/simple/key/main.go`

**Options**:

- `-w, --write`: Overwrite existing key in configuration without confirmation

**Behavior**:

1. Show key backup warning (unless `crypt.warned: true`)
2. Generate 32-byte key using crypto/rand
3. Encode using base58 (Bitcoin alphabet)
4. Output to stdout, or to configuration if `-w` / `--write` is given

#### 3.3 `belt crypt wireguard` - WireGuard Key Generation

**Location**: `src/commands/crypt/wireguard/main.go`

**Behavior**:

1. Generate X25519 keypair
2. Encode keys using standard base64 for WireGuard

Output format:

```plaintext
Private key : <base64-private-key>
Public key  : <base64-public-key>
```

### 4. `belt dns` - DNS Operations

#### 4.1 `belt dns flush` - Flush DNS Cache

**Location**: `src/commands/dns/flush/main.go`

**Behavior**:

1. Detect operating system using runtime.GOOS
2. Execute platform-specific command:
   - Windows: `ipconfig /flushdns`
   - macOS: `sudo dscacheutil -flushcache && sudo killall -HUP mDNSResponder`
   - Linux: `sudo systemd-resolve --flush-caches`
3. Output command being executed to stderr
4. Show success/failure with command output, with pretty output
5. Handle sudo requirement on Unix systems - it may ask for a password or it may just permit sudo invocation.

#### 4.2 `belt dns lookup <query> [record_type]` - DNS Lookup

**Location**: `src/commands/dns/lookup/main.go`

**Arguments**:

- `query`: Domain name or IP to query
- `record_type`: DNS record type (default: "A")

**Options**:

- `-e, --server`: DNS server to use (default: from config or "1.1.1.1")
- `-t, --root`: Use root servers directly (default: no, yes if specified)

**Behavior**:

1. Use specified or configured DNS server
2. Perform DNS resolution
3. Output raw DNS record data
4. Support all standard record types

**Dependencies**:

- `miekg/dns` for DNS operations

#### 4.3 `belt dns sec` - DNSSEC Validation

**Location**: `src/commands/dns/sec/main.go`

**Arguments**:

- `domain`: Root domain to check

**Behavior**:

1. Check `domain` domain
2. Query NS records to find authoritative nameserver
3. Resolve nameserver IP
4. Query DNSKEY with DNSSEC enabled
5. Validate DNSKEY using RRSIG
6. Give pretty output, showing these situations among any other I'm missing. These are not necessarily the exact text to output:
   - "Query failed: server error or no DNSKEY record"
   - "Query failed: unknown error"
   - "DNSKEY validation failed"
   - "DNSKEY validated OK"

### 5. Unimplemented Commands (Placeholder)

These commands should return a "Not yet implemented" message:

- `belt domain expiry`
- `belt domain ns`
- `belt tls cert req`
- `belt tls cert selfsign`
- `belt tls ciphers`

## Shared Components and Utilities

### Location: `src/internal/`

#### 1. Configuration Manager (`config/`)

- Load/save YAML configuration
- Handle environment variable precedence
- Manage warning states
- XDG path resolution

#### 2. Cryptography Package (`crypto/`)

- ChaCha20Poly1305 wrapper functions
- BLAKE2b hashing utilities
- Base58 encoding/decoding (Bitcoin alphabet)
- Secure random generation
- X25519 key generation

#### 3. Platform Detection (`platform/`)

- OS detection utilities
- Platform-specific command execution
- Sudo handling for Unix systems

#### 4. Output Formatting (`output/`)

- Charmbracelet lipgloss integration
- Consistent error formatting (stderr)
- Data output helpers (stdout)
- Warning display system

#### 5. Audio Utilities (`audio/`)

- File extension detection
- Metadata extraction wrapper
- Directory walking with filters

#### 6. DNS Utilities (`dns/`)

- DNS resolver configuration
- DNSSEC validation helpers
- Record formatting

## Library Dependencies

### Core Libraries (from scaffold)

- `github.com/alecthomas/kong` - CLI parsing
- `github.com/knadh/koanf/v2` - Configuration management
- `github.com/charmbracelet/bubbletea` - TUI framework
- `github.com/charmbracelet/huh` - Interactive prompts
- `github.com/charmbracelet/lipgloss` - Terminal styling

### Additional Required Libraries

- `github.com/adrg/xdg` - XDG BaseDirectory paths
- `gopkg.in/yaml.v3` - YAML parsing
- `github.com/btcsuite/btcutil` - Base58 encoding (Bitcoin alphabet)
- `golang.org/x/crypto` - ChaCha20Poly1305, BLAKE2b, X25519
- `github.com/miekg/dns` - DNS operations
- `github.com/dhowden/tag` - Audio metadata extraction

Pull in any others you need. Dependencies are good, implementing things by hand is bad. I don't care how big our dependency graph gets.

## Testing Requirements

### Compatibility Tests

1. Create test fixtures with encrypted data
2. Verify utility can decrypt correctly
3. Test YAML config compatibility
4. Verify output format matching

### Unit Tests

1. Cryptographic operations (encrypt/decrypt cycle)
2. Base58 encoding/decoding
3. Configuration loading/saving
4. Platform detection
5. DNS operations
6. Audio metadata extraction

### Integration Tests

1. Full command execution
2. Stdin/stdout handling
3. Error scenarios
4. Cross-platform behavior

## Critical Implementation Notes

### Code style

- Write idiomatic Go at all times.
- Preserve beautiful output with visual flair (spinners, boxes, progress indicators, etc.) wherever possible.
- I am not concerned about the size of the dependency graph, and I am eager to use libraries instead of implementing things from scratch.
- Add packages freely, just make sure you add them at their latest version, and that they have a commit in the last 6 months.
  - Remember, `context7` and `deepWiki` can help you learn about a library.

### Common Arguments

The arguments that are common to all utilities should have an effect wherever possible. Be intelligent about how they might be used, and add functionality for them.

This includes respecting the `-p` / `--pipe` option and writing structured JSON to `stdout`.

### Output Format Preservation

- Audio info: `filepath:samplerate:bitdepth:filesize`
- No extra whitespace or formatting
- Errors to stderr, data to stdout
- Script mode flags change output format

### Configuration Precedence

1. Environment variable (if specified in config)
2. Config file value
3. Default value

### Platform Considerations

- Use runtime.GOOS for OS detection
- Handle sudo requirements gracefully
- Support case-insensitive file extensions on all platforms

### Visual flair

- Make it pretty! I want plenty of visual flair, and you have the `charmbracelet` libraries to do that.
- Ideally create shared helpers for visual flair so we can reuse them in future work.

### Additional

- The date is 21 June, 2025.

### Documentation

When finished, document EVERYTHING -

- `README.md` - for humans. Write in a friendly, slightly sardonic way.
- `CLAUDE.md` - specifically for Claude Code agent. Prioritise information density and clarity to get across the information but keep token count reasonable. Assume Claude Code will be the consumer.
- `AGENTS.md` - for AI agents OTHER than Claude Code. Same rules as `CLAUDE.md` apply, but don't assume Claude Code will be the consumer.
- `TODO.md` - for humans. If there's anything you don't fully implement, or stub, or mock, document it here so that we can come back to it later.
  - ALL STUBS, MOCKS, FAKE DATA, AND SIMILAR **MUST** BE DOCUMENTED HERE.
  - THEY **MUST** ALSO BE DOCUMENTED IN TODO COMMENTS IF THERE IS A CODE LOCATION FOR THEM.

`README.md`, `CLAUDE.md`, and `AGENTS.md` should contain the same information, just presented in different ways.
