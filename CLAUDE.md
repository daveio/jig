# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Essential Commands

### Development

- `bun start` - Run the CLI directly from source
- `bun run build` - Build the project (TypeScript compilation + templates)
- `bun run test` - Run Jest tests with coverage
- `bun run test:jest:watch` - Run tests in watch mode

### Code Quality

- `bun run lint` - Run all linters (Biome, Trunk, TypeScript)
- `bun run lint:fix` - Auto-fix linting issues
- `bun run format` - Format code with Prettier, Biome, and Trunk
- `bun run lint:types` - TypeScript type checking only

### Testing Specific Commands

- `bun run test:jest` - Run Jest tests only
- `bun run test:jest:snapshot` - Update Jest snapshots

## Architecture

This is a CLI toolbelt built with the **Gluegun** framework. Key architectural points:

### CLI Framework (Gluegun)

- Entry point: `src/cli.ts` - creates the CLI runtime with Gluegun's `build()` API
- Commands are auto-discovered from `src/commands/` directory
- Each command exports a GluegunCommand object with `name` and `run` properties
- Built-in support for help, version, and plugin system

### Command Structure

- **Default command**: `src/commands/belt.ts` - simple welcome message
- **Generate command**: `src/commands/generate.ts` (alias: `g`) - creates model files from EJS templates
- Template system uses EJS files in `src/templates/`

### Build System

- **Runtime**: Bun (v1.2.16) with Node.js compatibility (v22.16.0)
- **TypeScript**: Strict configuration with comprehensive type checking
- **Build process**: TypeScript compilation to `build/` + template copying
- **Module system**: ESNext modules with `verbatimModuleSyntax`

### Code Quality Stack

- **Biome**: Primary linter and formatter with custom rules
- **Trunk**: Additional linting and formatting
- **Prettier**: Code formatting with specific config (no semicolons, double quotes)
- **TypeScript**: Strict mode with advanced checking (noUncheckedIndexedAccess, etc.)

### Testing

- **Jest** with TypeScript support (`ts-jest`)
- Integration tests in `__tests__/cli-integration.test.ts`
- Tests run actual CLI commands using Gluegun's system utilities

### Key Dependencies

- `gluegun` - CLI framework
- `@anthropic-ai/sdk` - Anthropic API integration
- Various utilities: `axios`, `chalk`, `boxen`, `ora`, `sharp`

## Development Notes

### Project Structure

```
src/
├── cli.ts              # Main CLI entry point
├── commands/           # Auto-discovered command modules
├── extensions/         # CLI extensions
├── templates/          # EJS templates for generation
└── types.ts           # Type definitions
```

### Command Development

Commands follow Gluegun conventions:

- Export object with `name` and `run` function
- Access toolbox utilities via destructuring
- Use `parameters.first` for primary argument

### Template System

- EJS templates in `src/templates/`
- Generated files use props passed from commands
- Templates are copied to build during compilation
