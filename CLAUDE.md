# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Development Workflow

```bash
# Run the CLI directly
bun src/index.ts <command> [options]

# Development commands
bun run lint           # Full lint check (biome, trunk, types)
bun run lint:fix       # Auto-fix linting issues
bun run format         # Format code (prettier, biome, trunk)
bun run lint:types     # TypeScript type checking only

# Dependency management
bun install            # Install dependencies
bun run reset          # Clean install (removes node_modules and bun.lock)
```

### CLI Usage

```bash
# Test commands
bun src/index.ts hello world --language fr --formal
bun src/index.ts list --size --date --recursive
bun src/index.ts --help                    # Show all commands
bun src/index.ts <command> --help          # Command-specific help
```

## Architecture Overview

### Core Framework (Commander-based CLI)

- **Entry Point**: `src/index.ts` - Bun executable with shebang, auto-discovers commands
- **Command System**: Dynamic command loading from `src/commands/` directory
- **Shared Options**: All commands inherit 16 standard options (verbose, quiet, pipe, recursive, etc.)

### Key Architectural Components

**Command Registration (`src/lib/command.ts`)**

- `loadCommands()` - Auto-discovers `.ts`/`.js` files in commands directory
- `registerBeltCommand()` - Wraps Commander.js with shared options and error handling
- `BeltCommand` interface - Standardized command structure with context-based actions

**Output Management (`src/lib/output.ts`)**

- `OutputManager` class - Centralized output with levels (silent/quiet/normal/verbose)
- Supports multiple formats: console, JSON pipe, tables, spinners, progress bars
- Global instance via `getGlobalOutput()` accessible in all commands

**Command Context System (`src/lib/types.ts`)**

- `CommandContext` - Unified interface passed to all command actions
- Contains parsed shared options, command instance, and arguments
- `SharedOptions` - 16 standard CLI options available to all commands

### Command Development Pattern

New commands in `src/commands/` must export a `BeltCommand` object:

```typescript
import type { BeltCommand, CommandContext } from "../lib/types.ts"
import { getGlobalOutput } from "../lib/output.ts"

const myCommand: BeltCommand = {
  name: "my-command",
  description: "Command description",
  arguments: [{ name: "arg", description: "Argument desc", required: false }],
  options: [{ flags: "--flag", description: "Flag desc" }],
  action: async (context: CommandContext) => {
    const output = getGlobalOutput()
    const { args, options } = context
    const commandOpts = context.command.opts()

    // Access shared options: options.verbose, options.pipe, etc.
    // Access command-specific options: commandOpts.flag
    // Command implementation here
  },
}

export default myCommand
```

### Shared Options Architecture

Every command automatically receives these options:

- Output control: `--verbose`, `--quiet`, `--silent`, `--pipe`
- File operations: `--input`, `--output`, `--recursive`, `--force`
- Data processing: `--all`, `--list`, `--compress`

### Runtime Requirements

- **Bun-first**: Uses Bun APIs, includes runtime detection with friendly Node.js fallback
- **TypeScript**: Strict configuration with bundler module resolution
- **Import style**: ES modules with `.ts` extensions, `node:` protocol for builtins

### Tooling Stack

- **Formatting**: Triple-stack (Prettier → Biome → Trunk)
- **Linting**: Biome + Trunk + TypeScript compiler
- **Dependencies**: Bun package manager, includes CLI utilities (chalk, boxen, ora, commander)

### Output System Features

Commands can use the output manager for:

- Leveled logging (debug, info, success, warn, error)
- Structured data (tables, JSON for pipes)
- Interactive elements (spinners, progress bars)
- Formatted containers (boxes with borders)

The `--pipe` option switches all output to JSON format for command chaining.
