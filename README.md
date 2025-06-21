# Belt 🛠️

A modular CLI toolbelt that consolidates various utility scripts into a single, elegant command-line application. Because who needs 47 different scripts scattered across their system when you can have one tool to rule them all?

## Features

### 🔐 Cryptography Operations

- **Random data generation**: Hex strings and secure passwords
- **Simple encryption**: ChaCha20Poly1305 with BLAKE3 hashing for fast, secure encryption
- **WireGuard keys**: Generate X25519 keypairs for VPN configurations

### 🎵 Audio File Operations

- **Metadata extraction**: Get sample rate, bit depth, and file size from audio files
- **Batch processing**: Recursively analyze entire music directories
- **Format support**: FLAC, MP3, and M4A files

### 🌐 DNS Operations

- **DNS lookups**: Query any record type from any server
- **Cache flushing**: Platform-specific DNS cache clearing
- **DNSSEC validation**: Verify domain security with detailed diagnostics

### 🚧 Coming Soon

- Domain expiry checking
- TLS certificate generation
- And more based on what I need next week...

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/daveio/belt.git
cd belt

# Build with Go 1.24+
go build -o belt ./src

# Move to your PATH
sudo mv belt /usr/local/bin/
```

### First Run

```bash
# Initialize configuration
belt init

# IMPORTANT: Back up the encryption key it shows you!
```

## Usage

### Cryptography

```bash
# Generate random data
belt crypt random hex 32              # 64-character hex string
belt crypt random pw 20               # 20-character password

# Encrypt/decrypt files
echo "secret data" | belt crypt simple encrypt > data.enc
cat data.enc | belt crypt simple decrypt

# Generate WireGuard keys
belt crypt wire-guard
```

### Audio Operations

```bash
# Analyze single file
belt audio info song.mp3

# Analyze entire directory
belt audio info ~/Music

# JSON output for scripting
belt audio info ~/Music --pipe | jq '.path'
```

### DNS Operations

```bash
# Simple lookups
belt dns lookup example.com
belt dns lookup example.com MX
belt dns lookup example.com --server 8.8.8.8

# DNSSEC validation
belt dns sec cloudflare.com

# Flush DNS cache (platform-aware)
belt dns flush
```

## Configuration

Belt uses a YAML configuration file located at:

- macOS/Linux: `~/.config/belt/config.yaml`
- Windows: `%APPDATA%\belt\config.yaml`

### Example Configuration

```yaml
crypt:
  env: BELT_CRYPT_KEY
  key: your-base58-encoded-key-here
  warned: false

dns:
  server: 1.1.1.1
  root: false
```

### Environment Variables

- `BELT_CRYPT_KEY`: Override the encryption key from config
- `BELT_*`: Any config value can be overridden with environment variables

## Design Philosophy

1. **Beautiful output**: Life's too short for ugly terminal output
2. **Scriptable**: Every command supports `--pipe` for JSON output
3. **Secure defaults**: ChaCha20Poly1305 encryption, BLAKE3 hashing
4. **Cross-platform**: Works on macOS, Linux, and Windows
5. **No feature creep**: Does what it says on the tin, nothing more

## Technical Details

- Built with Go 1.24+ for performance and portability
- Kong for elegant command-line parsing
- Charmbracelet libraries for beautiful terminal UI
- Koanf for flexible configuration management
- Standard crypto libraries for security

## Why "Belt"?

Because it's a toolbelt. And I already have a project called "tools". Naming is hard.

## Contributing

Feel free to open issues or PRs. Keep in mind this is a personal tool that I'm sharing because someone might find it useful. If you want a feature, you're probably better off forking it.

## License

MIT - See LICENSE file for details.

---

_Built with ❤️ and probably too much coffee_
