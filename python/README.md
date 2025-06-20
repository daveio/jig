# `belt`: a cli toolbox

## Installation

### `pipx` (recommended)

I would suggest installation using `pipx`. This keeps `belt` from affecting anything else in your default `pip` environment.

First install `pipx`-

```shell
pip install pipx
```

Then use `pipx` to install `belt-cli`-

```shell
pipx install belt-cli
```

By default, this will put the executable in `~/.local/bin`- make sure it's in your shell`$PATH`or`fish_user_paths`.

### `pip`

If you don't want to use `pipx` then you can simply do

```shell
pip install belt-cli
```

## Configuration

### Path

```text
~/.config/belt/config.yaml
```

### Generating Configuration

To generate an initial configuration file containing defaults and a random key, run `belt init`. If a configuration file exists,`belt` will warn you and request confirmation.

You can also give permission to overwrite existing configuration with the `-o` or `--overwrite` flag.

### Sample Configuration

```yaml
crypt:
  env: BELT_CRYPT_KEY
  key: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  warned: false
dns:
  server: 1.1.1.1
  root: false
```

## Key Management

Make sure you **back up your key**, whether you store it in the config file or the environment. A password manager is an excellent choice for this.

If you use `chezmoi` to manage your dotfiles, add the `belt` config file with encryption. Though you then need to make sure you back up your `chezmoi` key.

Without the key, anything you encrypt with `belt crypt simple encrypt` will be unrecoverable.

## Encryption and decryption

Encryption is performed using `ChaCha20Poly1305` AEAD combined with a 64-bit `BLAKE2b` hash. Keys and encrypted data are `Base85` encoded with the Bitcoin alphabet for compatibility, which uses only alphanumeric characters. Key and data integrity are checked at multiple stages.

**The cryptographic implementation in `belt` does not split input into blocks**. This makes it entirely unsuitable for encrypting files or anything larger than around 100kB. It's intended for encrypting and decrypting short strings like API keys, but works fine for most configuration files too.

If you want strong symmetric encryption suitable for files, [`age`][age] (or the faster Rust implementation, [`rage`][rage]) is a good choice.

[age]: https://github.com/FiloSottile/age
[rage]: https://github.com/str4d/rage
