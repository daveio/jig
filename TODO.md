# TODO.md

## Implemented Features

All core features from the implementation plan have been successfully implemented:

- ✅ Configuration system with Koanf
- ✅ XDG-compliant config paths
- ✅ Shared components (crypto, platform, audio, DNS utilities)
- ✅ Beautiful terminal output with Charmbracelet libraries
- ✅ `belt init` - Configuration initialization
- ✅ `belt audio info` - Audio metadata extraction
- ✅ `belt crypt random hex/pw` - Random data generation
- ✅ `belt crypt simple encrypt/decrypt/key` - Encryption operations
- ✅ `belt crypt wire-guard` - WireGuard key generation
- ✅ `belt dns flush/lookup/sec` - DNS operations
- ✅ Placeholder commands for future features

## Bug Fixes

### Critical Issues

- **TODO**: (a1b2c3) Fix memory exhaustion in decrypt command - use streaming instead of io.ReadAll(), match encrypt behaviour
- **TODO**: (b2c3d4) Fix potential panic in config.Get() - ensure Load() is called before Get()
- **TODO**: (c3d4e5) Add nonce length validation in DecryptBlock() before aead.Open()

### Security Issues

- **TODO**: (d4e5f6) Fix partial encryption on read errors in encrypt command
- **TODO**: (e5f6g7) Add proper DNS server address validation in NewResolver()
- **TODO**: (f6g7h8) Add timeout handling for all network operations

### Error Handling

- **TODO**: (g7h8i9) Propagate file close errors in init command instead of just logging
- **TODO**: (h8i9j0) Add nil checks for config in output.go print methods
- **TODO**: (i9j0k1) Add proper error handling for type assertions in dns/lookup.go

### Platform Issues

- **TODO**: (j0k1l2) Fix Windows compilation issue with os.Geteuid() in platform.go
- **TODO**: (k1l2m3) Fix race condition in audio.go between file open and stat

### Resource Management

- **TODO**: (l2m3n4) Add bounds checking for all array/slice operations
- **TODO**: (m3n4o5) Use sync.Once for one-time config initialization
- **TODO**: (n4o5p6) Add context.Context support for proper cancellation handling

## Known Limitations

### Audio Metadata Extraction

- The `dhowden/tag` library only provides metadata tags (artist, album, etc.), not technical properties
- Sample rate and bit depth are hardcoded to common defaults (44100 Hz, 16-bit)
- **TODO**: (d4a8c9) Find or implement proper audio property extraction
  - Consider using format-specific libraries (go-flac, go-mp3, etc.)
  - Or shell out to `ffprobe` if available

### Test Coverage

- **TODO**: (b7e2f5) Write unit tests for crypto operations
- **TODO**: (c3d9a1) Write tests for platform detection
- **TODO**: (e5f6b8) Write tests for configuration loading
- **TODO**: (a9c4d2) Write integration tests for commands
- **TODO**: (f1e7b3) Add test fixtures for encrypted data compatibility

## Future Enhancements

### Domain Operations

- **TODO**: (8a5c6d) Implement `belt domain expiry` using WHOIS
- **TODO**: (9b7e8f) Implement `belt domain ns` for nameserver checking

### TLS Operations

- **TODO**: (4c3d2a) Implement `belt tls cert req` for CSR generation
- **TODO**: (5d4e3b) Implement `belt tls cert selfsign` for self-signed certs
- **TODO**: (6e5f4c) Implement `belt tls ciphers` to list supported cipher suites

### Additional Features

- **TODO**: (7f6a5d) Add compression support (`--compress` flag)
- **TODO**: (1a2b3c) Add file input/output support (`--input`/`--output` flags)
- **TODO**: (2b3c4d) Add verbose logging (`--verbose` flag)
- **TODO**: (3c4d5e) Add progress indicators for long operations
- **TODO**: (d5e6f7) Add update checker for new versions

### Security Enhancements

- **TODO**: (e6f7a8) Add key derivation from passphrase option
- **TODO**: (f7a8b9) Add support for multiple encryption keys
- **TODO**: (a8b9c0) Add key rotation functionality
- **TODO**: (b9c0d1) Add secure key storage integration (system keychain)

### Platform Support

- **TODO**: (c0d1e2) Test and fix Windows-specific issues
- **TODO**: (d1e2f3) Add PowerShell completion support
- **TODO**: (e2f3a4) Add shell completion for bash/zsh/fish

## Code Quality

### Refactoring Opportunities

- **TODO**: (f3a4b5) Extract common command patterns to base struct
- **TODO**: (a4b5c6) Consolidate error handling patterns
- **TODO**: (b5c6d7) Add context timeout support for long operations

### Documentation

- **TODO**: (c6d7e8) Add inline code examples to command help
- **TODO**: (d7e8f9) Create man pages
- **TODO**: (e8f9a0) Add architecture diagram to docs

## Notes

- All placeholder commands return "not yet implemented" error
- Encryption block size (1024 bytes) chosen for streaming support
- Base58 encoding uses Bitcoin alphabet for consistency
- BLAKE3 configured for 64-byte output to match original spec
