# Implementation Plan: `jig`

## Layout

- Base CLI framework with `clap`
  - <https://github.com/clap-rs/clap>
  - Supports command shortening to disambiguation
    - Derive API: `#[command(infer_subcommands = true)]`
  - `jig fmt` is an alias for `jig format`
- Terminal UI with `ratatui`
  - <https://github.com/ratatui/ratatui>
- Encryption and decryption with `age` via `rage`
  - <https://github.com/str4d/rage>


## Command Tree

```mermaid
graph TD
  A[jig] --> 1(crypt)
  A --> 3(generate)
  A --> 4(network)
  4 --> 4A(dns)
  A --> 5(format)
  A --> 6(convert)
  A --> 7(mcp)
```
