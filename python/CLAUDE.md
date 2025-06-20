# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Belt is a command-line toolbox written in Python that provides various utility functions for:

- Cryptography (encryption/decryption, random generation, WireGuard keypairs)
- DNS operations (lookups, DNSSEC checks, cache flushing)
- TLS operations (certificate generation, cipher inspection)
- Domain operations (expiry checking, nameserver lookup)
- Audio file inspection

The codebase uses Click for CLI command structure and is designed with modular command files that are imported into the main CLI entrypoint.

## Development Environment

### Setup

1. Install dependencies using Poetry:

```bash
poetry install
```

2. For local development without installing:

```bash
poetry shell
python -m src.belt_cli
```

### Building the Package

Use Poetry to build the package:

```bash
poetry build
```

### Publishing

This project uses GitHub Actions for CI/CD, which handles:

- PyPI publishing
- Docker image building and publishing to DockerHub and GitHub Container Registry
- Sigstore signing for releases

## Code Architecture

The application follows a modular structure:

- `belt_cli.py`: Main entrypoint that defines the CLI command structure using Click
- `config.py`: Handles configuration file management (location, reading, default generation)
- `cryptor.py`: Core cryptography class for encryption/decryption operations
- Command modules:
  - `audio_commands.py`: Audio file inspection commands
  - `crypt_commands.py`: Cryptography commands (random generation, encryption/decryption)
  - `dns_commands.py`: DNS-related operations
  - `domain_commands.py`: Domain-related operations
  - `tls_commands.py`: TLS-related operations

### Key Features

1. **Configuration Management**: Uses XDG BaseDirectory for configuration storage (~/.config/belt/config.yaml)
2. **Cryptography**: ChaCha20Poly1305 AEAD combined with BLAKE2b hash and Base58 encoding
3. **CLI Interface**: Extensive use of Click for command structure, arguments and options

## Development Guidelines

1. **Command Structure**: Follow the existing pattern where:

   - Commands are defined in `belt_cli.py`
   - Implementation is placed in a separate module named `*_commands.py`
   - Each command function returns a string or performs output with Click's echo

2. **Configuration Access**: Use the functions in `config.py` to access configuration

3. **Error Handling**: Use Click's echo with `err=True` for error messages

4. **Documentation**: Document parameters and return values using docstrings
