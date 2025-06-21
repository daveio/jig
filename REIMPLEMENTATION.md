# Belt CLI - Python Implementation Behavior Documentation

This document describes the complete behavior of the Python Belt CLI application for reimplementation in the Bun/TypeScript CLI framework. Focus is on BEHAVIOR, not implementation details.

## Application Overview

Belt is a modular CLI toolbelt that provides utilities for cryptography, DNS operations, TLS management, domain operations, and audio file inspection. The application follows a hierarchical command structure with shared configuration management.

### Entry Point Behavior

- Main command: `belt`
- Uses Click framework command groups for hierarchical structure
- Supports `--version` flag at top level
- Auto-discovers and loads command modules
- All commands inherit from base CLI configuration

## Configuration System

### Configuration File Location

- Path: `~/.config/belt/config.yaml` (XDG BaseDirectory compliant)
- Format: YAML
- Auto-created directory structure if missing

### Configuration Structure

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

- Environment variable specified in `crypt.env` takes precedence over `crypt.key`
- Missing configuration sections default to safe values
- Config loading fails gracefully with helpful error messages
- Key generation uses cryptographically secure random generation

## Command Structure

### Top-Level Commands

#### `belt init`

**Purpose**: Initialize configuration file
**Arguments**: None
**Options**:

- `-o, --overwrite`: Overwrite existing config without confirmation
  **Behavior**:
- Checks if config file exists
- If exists and no `--overwrite`: prompts for confirmation
- Generates new config with random encryption key
- Creates directory structure if needed
- Writes default YAML configuration

#### `belt audio`

**Purpose**: Audio file operations (command group)

##### `belt audio info <path>`

**Arguments**:

- `path`: File or directory path to analyze
  **Behavior**:
- Recursively walks directory structure
- Identifies audio files by extension: `.flac`, `.mp3`, `.m4a`
- For each audio file, outputs: `{filepath}:{sample_rate}:{bits_per_sample}:{file_size}`
- MP3 files default to 16-bit depth (format limitation)
- Uses mutagen library for metadata extraction
- Outputs one line per audio file found

#### `belt crypt`

**Purpose**: Cryptography operations (command group)

##### `belt crypt random`

**Purpose**: Random data generation (subgroup)

###### `belt crypt random hex [length]`

**Arguments**:

- `length`: Number of bytes (default: 16)
  **Behavior**:
- Generates cryptographically secure random hex string
- Output length is `length * 2` characters
- Uses system entropy source

###### `belt crypt random pw [length]`

**Arguments**:

- `length`: Password length (default: 16)
  **Behavior**:
- Generates secure password with mixed character types
- Character set: letters + digits + `-_.@#$%&*+=:`
- Ensures at least one digit and one punctuation character
- First character is always a digit
- Last character is always punctuation
- Middle characters are random from full alphabet

##### `belt crypt simple`

**Purpose**: Simple encryption/decryption (subgroup)

###### `belt crypt simple encrypt`

**Input**: Reads plaintext from stdin (binary mode)
**Output**: Base58-encoded ciphertext to stdout
**Behavior**:

- Shows warning about key backup (unless `crypt.warned: true`)
- Uses ChaCha20Poly1305 AEAD encryption
- Generates random 12-byte nonce per encryption
- Computes BLAKE2b 64-byte hash of plaintext for integrity
- Structure: `nonce(12) + hash(64) + ciphertext`
- Final output is base58-encoded using Bitcoin alphabet
- Requires valid encryption key from config or environment

###### `belt crypt simple decrypt`

**Input**: Reads base58-encoded ciphertext from stdin
**Output**: Decrypted plaintext to stdout (binary mode)
**Behavior**:

- Shows warning about key backup (unless `crypt.warned: true`)
- Decodes base58 input to binary
- Extracts nonce (first 12 bytes), hash (next 64 bytes), ciphertext (remainder)
- Decrypts using ChaCha20Poly1305 with nonce and hash as additional data
- Verifies integrity by recomputing BLAKE2b hash of decrypted data
- Fails with error if hash verification fails
- Outputs raw binary data to stdout

###### `belt crypt simple key`

**Behavior**:

- Shows warning about key backup (unless `crypt.warned: true`)
- Generates new ChaCha20Poly1305 key (32 bytes)
- Outputs base58-encoded key to stdout
- Uses cryptographically secure random generation

##### `belt crypt wireguard`

**Options**:

- `-s, --script`: Output format for scripting (`PRIVATE PUBLIC`)
  **Behavior**:
- Generates X25519 keypair for WireGuard
- Default output format:
  ```
  Private key : <base64-private-key>
  Public key  : <base64-public-key>
  ```
- Script format: `<private-key> <public-key>` (space-separated)
- Keys are base64-encoded (standard encoding, not base58)

#### `belt dns`

**Purpose**: DNS operations (command group)

##### `belt dns flush`

**Behavior**:

- Detects operating system automatically
- Executes platform-specific DNS cache flush commands:
  - Windows: `ipconfig /flushdns`
  - macOS: `sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder`
  - Linux: `sudo systemd-resolve --flush-caches`
- Outputs command being executed to stderr
- Shows success/failure message with command output
- Requires sudo privileges on Unix systems

##### `belt dns lookup <query> [record_type]`

**Arguments**:

- `query`: Domain name or IP to query
- `record_type`: DNS record type (default: "A")
  **Options**:
- `-s, --server`: DNS server to use (default: from config or "1.1.1.1")
- `-r, --root`: Use root servers directly (boolean flag)
  **Behavior**:
- Performs DNS resolution using specified or configured server
- Outputs raw DNS record data
- Supports all standard DNS record types
- Uses dnspython for resolution

##### `belt dns sec`

**Behavior**:

- Hardcoded to check DNSSEC validation for "dave.io" domain
- Queries for NS records to find authoritative nameserver
- Resolves nameserver to get IP address
- Queries for DNSKEY record with DNSSEC enabled
- Validates DNSKEY signature using RRSIG record
- Output states:
  - "QUERY FAILED (SERVER ERROR OR NO DNSKEY RECORD)"
  - "SOMETHING WENT WRONG"
  - "DNSKEY VALIDATION FAILED"
  - "DNSKEY VALIDATED OK"

#### `belt domain`

**Purpose**: Domain operations (command group)

##### `belt domain expiry`

**Behavior**: Returns "domain_expiry: Not yet implemented"

##### `belt domain ns`

**Behavior**: Returns "domain_ns: Not yet implemented"

#### `belt tls`

**Purpose**: TLS operations (command group)

##### `belt tls cert`

**Purpose**: Certificate operations (subgroup)

###### `belt tls cert req`

**Behavior**: Returns "tls_cert_req: Not yet implemented"

###### `belt tls cert selfsign`

**Behavior**: Returns "tls_cert_selfsign: Not yet implemented"

##### `belt tls ciphers`

**Behavior**: Returns "tls_ciphers: Not yet implemented"

## Cryptographic Implementation Details

### Key Management

- **Key Size**: 32 bytes (256-bit)
- **Key Encoding**: Base58 with Bitcoin alphabet for compatibility
- **Key Generation**: Uses `ChaCha20Poly1305.generate_key()` for cryptographic security
- **Key Sources**: Environment variable (priority) > config file key

### Encryption Process

1. **Algorithm**: ChaCha20Poly1305 Authenticated Encryption with Associated Data (AEAD)
2. **Nonce**: 12 random bytes generated per encryption
3. **Integrity**: BLAKE2b hash (64 bytes) of plaintext computed before encryption
4. **Associated Data**: BLAKE2b hash used as associated data for AEAD
5. **Output Structure**: `nonce || hash || ciphertext`
6. **Final Encoding**: Base58 with Bitcoin alphabet

### Security Features

- **Integrity Verification**: Double-checked via AEAD and separate hash verification
- **Key Backup Warning**: Displayed unless explicitly acknowledged in config
- **No Block Mode**: Designed for small data (API keys, config), not files
- **Authenticated Encryption**: Prevents tampering and ensures authenticity

## Error Handling Patterns

### Configuration Errors

- Missing config file: Clear message directing to `belt init`
- Invalid YAML: Standard YAML parsing error messages
- Missing key: Error when attempting crypto operations

### Cryptographic Errors

- Invalid ciphertext: "Invalid ciphertext" error with hash mismatch
- Missing key: Fails gracefully with configuration guidance
- Invalid base58: Standard decoding error handling

### Platform-Specific Behavior

- DNS flush: OS detection with appropriate command selection
- Path handling: Cross-platform path resolution
- Audio file detection: Case-insensitive extension matching

## Implementation Notes for Future Self

### Critical Patterns to Preserve

1. **Configuration precedence**: Environment variables override config file values
2. **Cryptographic structure**: Exact nonce + hash + ciphertext layout for compatibility
3. **Base58 encoding**: Must use Bitcoin alphabet for existing data compatibility
4. **Warning system**: Key backup warnings with config-based acknowledgment
5. **Hierarchical commands**: Maintain exact command structure for user familiarity

### Security Considerations

1. **Key generation**: Must use cryptographically secure random sources
2. **Memory handling**: Consider secure memory handling for keys in TypeScript
3. **Input validation**: Validate all user inputs, especially for crypto operations
4. **Error messages**: Don't leak sensitive information in error outputs

### Platform Compatibility

1. **XDG BaseDirectory**: Implement equivalent for config path resolution
2. **DNS flush commands**: Maintain exact command strings for OS compatibility
3. **Audio file formats**: Support same extensions with equivalent metadata extraction
4. **Path handling**: Ensure cross-platform directory traversal works identically

### Output Format Preservation

1. **Audio info**: Exact format `filepath:samplerate:bitdepth:filesize`
2. **WireGuard keys**: Maintain both human-readable and script formats
3. **Error outputs**: Use stderr for informational messages, stdout for data
4. **DNS responses**: Preserve raw record output format

### Dependencies to Replace

- **Click**: Replace with Commander.js (already in project)
- **dnspython**: Replace with native Node.js/Bun DNS APIs or equivalent library
- **cryptography**: Replace with Node.js crypto module or equivalent for ChaCha20Poly1305
- **mutagen**: Replace with Node.js audio metadata library
- **base58**: Replace with TypeScript base58 implementation
- **XDG BaseDirectory**: Replace with Node.js equivalent or manual implementation
- **YAML**: Replace with existing YAML parser in project dependencies

### Testing Considerations

1. **Crypto compatibility**: Ensure encrypted data from Python version decrypts correctly
2. **Config compatibility**: Same YAML structure and default generation
3. **Command output**: Identical output formats for all implemented commands
4. **Error scenarios**: Same error messages and exit codes where applicable
