# Implementation Plan: `jig`

## Layout

- Base CLI framework with `clap`
  - <https://github.com/clap-rs/clap>
- Terminal UI with `ratatui`
  - <https://github.com/ratatui/ratatui>
- Encryption and decryption with  `rage`
  - <https://github.com/str4d/rage>
- Terminal spinners from `spinoff`
  - <https://github.com/ad4mx/spinoff>
- [Easter Egg](#easter-egg) with ludicrous effects

## Command Tree

```mermaid
graph TD
  A[jig] --> 1(crypt)
  A --> 3(generate)
    3 --> 3A(hex)
    3 --> 3B(password)
    3 --> 3C(key)
      3C --> 3C1(crypt)
      3C --> 3C2(wireguard)
      3C --> 3C3(ssh)
  A --> 4(network)
    4 --> 4A(dns)
  A --> 5(format)
  A --> 6(convert)
  A --> 7(mcp)
```

# Notes

- `clap` supports command shortening to disambiguation
  - Derive API: `#[command(infer_subcommands = true)]`
- `jig fmt` is an alias for `jig format`
- `tachyonfx` integrates nicely with `ratatui`

# Easter Egg

Use TachyonFX as the primary effects engine.

-  Use TUI-Rain for atmospheric background effects.
-  Layer in Firework-RS for explosive moments.
-  Use RASCII to convert a photo of yourself into animated ASCII art.
-  Add Rusty TermColor typewriter effects for text reveals.
-  Finish with Spinoff spinners for loading sequences.

### Core Effects Engine

`tachyonfx`

- <https://github.com/junkdog/tachyonfx>
- The primary shader-like effects engine for ratatui applications
- A ratatui library for creating shader-like effects in terminal UIs with color transformations, animations, and complex effect combinations

### Atmospheric Effects

`tui-rain`

- <https://github.com/levilutz/tui-rain>
- Rain and atmospheric effects widget
- A simple stateless ratatui widget that generates various rain effects including Matrix rain, normal rain, snow, and emoji floods

### Explosive Visual Effects

`firework-rs`

- <https://github.com/Wayoung7/firework-rs>
- ASCII art firework simulator
- A cross-platform ASCII-art firework simulator that provides colorful displays, smooth animations, and a simple particle system

### ASCII Art Generation

`rascii_art`

- <https://github.com/UTFeight/RASCII>
- Advanced image to ASCII art converter
- An advanced image to ASCII art tool and crate that supports colored ASCII generation, custom dimensions, and multiple character sets including block, emoji, default, russian, and slight variants

### Text Effects

`rusty-termcolor`

- <https://github.com/rusty-libraries/rusty-termcolor>
- Terminal text formatting and effects
- A lightweight terminal manipulation library that provides color manipulation, text effects (typewriter, loading bar, wiggle, matrix), text formatting, and terminal control functions

### Progress Indicators

`spinoff`

- <https://github.com/ad4mx/spinoff>
- Terminal spinner library
- An easy-to-use, robust library for displaying spinners in the terminal with over 80 spinner variants, custom colors, and multiple output stream support
