# `jig`

## About

`jig` is a tool to manage various aspects of my development environment.

It auto-detects languages, package managers, and configurations to tweak.

**Language:** Rust

**Template engine:** Ask the `context7` MCP tool. Find something flexible and sensible.

**Git support:** Use a `git` library instead of shelling out. Ask the `context7` MCP tool for one.

## Core Concepts

### Auto-Detection

The tool employs pattern recognition to identify:

- Programming languages through file markers (package.json, Cargo.toml, etc.)
- Package managers through lockfiles and manifests
- Configuration formats through file extensions
- Version control through repository markers

### Template Composition

Templates are composed from shared and language-specific components:

- Shared components contain cross-language patterns
- Language templates reference shared components via placeholder files
- The system resolves and combines components at runtime

### Ecosystem Management

Each programming ecosystem has its own conventions:

- Package managers (npm, cargo, pip, bundler, etc.)
- Dependency files (package.json, Cargo.toml, requirements.txt, etc.)
- Build tools and scripts
- Testing frameworks

### Configuration Synchronization

The tool maintains consistency across:

- Development tool configurations
- CI/CD pipeline definitions
- Dependency management policies
- Code quality standards

## Template System

_[imagine a fox juggling template files, with shared components glowing in the center]_

### Directory Structure

We've organized templates with a shared component system to reduce duplication:

```plaintext
templates/
├── shared/             # Shared components
│   ├── gitignore/      # Common gitignore patterns
│   │   ├── common.gitignore     # Patterns common to all projects
│   │   ├── python.gitignore     # Python-specific patterns
│   │   └── rust.gitignore       # Rust-specific patterns
│   └── github/         # Shared GitHub workflows
│       └── workflows/  # GitHub Actions workflows
│           ├── base.yml.tera           # Base workflow template
│           ├── rust_jobs.yml.tera      # Rust-specific jobs
│           └── python_jobs.yml.tera    # Python-specific jobs
├── rust/               # Rust-specific templates
├── python/             # Python-specific templates
└── ... more languages
```

### Placeholder Files

Each language directory uses placeholder files to reference shared components:

1. **Gitignore placeholders**:

   - A file named `gitignore` contains lines listing which gitignore files to include
   - Example: `common\npython` includes both common and python patterns

2. **Workflow placeholders**:
   - A file named `workflows` contains YAML defining workflow parameters
   - Example:
     ```yaml
     base:
       workflow_name: Python
       jobs_template: python_jobs
     ```

### Template Filenames

Template filenames must not contain template syntax (like `{{ variable }}`). Instead:

1. Use static placeholders (e.g., `PROJECT_NAME.gemspec.tera`) in filenames
2. Place all template logic inside the file content
3. The system will handle the translation of these placeholders based on the project context

This approach improves maintainability and makes templates easier to work with, while still allowing for dynamic file generation.

### Implementation

The template system is implemented with:

1. A component module (`src/template/component.rs`) that:

   - Detects placeholder files
   - Processes different component types
   - Handles component-specific logic

2. Updates to the template module (`src/template/mod.rs`) to:

   - Detect and process placeholder files
   - Handle shared components correctly
   - Generate the final files

3. Tests (`src/template/tests.rs`) that verify:
   - Placeholder file detection
   - Component processing for gitignore and workflows

This system maintains backward compatibility while making templates more maintainable.

## Subcommands

### `jig new [language]`

- Create a new repository using the `[language]` programming language.
- Initialise repository for `git` as the very first task.
  - As changes are made, `commit` them after each logical segment.
  - Don't `push`.
- Explore my Baseline repository in `/Users/dave/src/github.com/daveio/_baseline` to learn about things we can set up.
  - Create a new template archive from the files there.
  - Note that `rules/` is a Git submodule - when using files from it, we should check for updates.
- Supports `--dry-run` to only explain what would be changed.

### `jig update [repository]`

- Updates files in `[repository]` from a potentially changed template set.
- `jig update` without `[repository]` updates the current repository.
- Informs user if there are no changes.
- Supports `--dry-run` to only explain what would be changed.

### `jig ai [tool]`

- Sets up the AI support in `[tool]` using the rules and MCP servers we have configured.
- `jig ai` without `[tool]` configures all tools.
- Tools:
  - Claude Desktop
  - Cursor editor
  - Zed editor
  - Goose
- Supports `--dry-run` to only explain what would be changed.

### `jig bump [repository] [--ecosystem <ecosystem>]`

- Bumps versions to latest, in various package managers and configuration files.
- Explore all the repositories in `/Users/dave/src` to find package managers and configuration files to set up.
- Python should always use `uv` and migrate to it if anything else is used (for example `poetry`).
  - Use `PEP-518` keys in `pyproject.toml` . See [https://peps.python.org/pep-0518](https://peps.python.org/pep-0518/)
- `jig bump` without `[repository]` updates the current repository.
- Include GitHub Actions. Fetch the latest commit for the repository on `main` (and `master` if `main` doesn't exist) and update them to use it, even if they're configured for a tag.
- Ecosystem-specific updates allow targeted dependency management:
  - `--ecosystem node`: Update only Node.js/npm packages
  - `--ecosystem python`: Update only Python packages
  - `--ecosystem rust`: Update only Rust packages
  - `--ecosystem ruby`: Update only Ruby packages
  - `--ecosystem java`: Update only Java packages
  - `--ecosystem go`: Update only Go packages
  - `--ecosystem actions`: Update only GitHub Actions workflows
- Supports `--cached` flag to use offline cache when available

### `jig dependabot [repository]`

- Manages automated dependency update configuration
- Scans repository to detect all package ecosystems in use
- Creates or updates `.github/dependabot.yml` configuration file
- Configures daily update schedules for each detected ecosystem
- Detects ecosystems by presence of:
  - Package manifests (package.json, Cargo.toml, etc.)
  - Lock files (package-lock.json, Cargo.lock, etc.)
  - Language-specific configuration files
  - GitHub Actions workflows
- `jig dependabot` without `[repository]` configures current repository
- Supports `--dry-run` to preview changes

## Architecture Principles

### Modularity

Each command operates independently with clear boundaries:

- Commands handle user interaction and orchestration
- Modules provide reusable functionality
- Utilities offer cross-cutting concerns

### Extensibility

New capabilities can be added through:

- Additional language templates
- New package manager support
- Extended tool integrations
- Custom ecosystem handlers

### Idempotency

All operations should be safely repeatable:

- Check existing state before making changes
- Report when no changes are needed
- Preserve user customizations where possible

### Transparency

Users should understand what the tool does:

- Dry-run mode shows planned changes
- Verbose output explains decisions
- Clear commit messages document changes

## Future Enhancements

Potential future improvements to the template system:

1. Additional component types beyond gitignore and workflows
2. More granular control over component inclusion
3. Version-specific components for different language versions

## Quality Assurance

### Testing Requirements

The system must include comprehensive test coverage for:

1. **Configuration System**

   - XDG Base Directory specification compliance
   - Default value generation and validation
   - Configuration persistence and loading
   - Environment variable support

2. **Template Processing**

   - Component detection and validation
   - Placeholder file resolution
   - Template rendering accuracy
   - Cross-platform path handling

3. **Version Control Integration**

   - Repository initialization and management
   - Commit signature creation
   - Environment variable precedence
   - Branch operations

4. **Ecosystem Detection**

   - Package manager identification
   - Configuration file parsing
   - Multi-ecosystem projects
   - Missing dependency handling

5. **Utility Functions**
   - Path normalization and validation
   - File system operations
   - YAML processing
   - Error handling and recovery

### Test Environment

Tests should:

- Use isolated temporary directories
- Mock external dependencies where appropriate
- Test both success and failure scenarios
- Validate cross-platform compatibility
- Ensure idempotent operations

### Error Handling

All operations must:

- Return structured error information
- Provide clear user-facing messages
- Include context for debugging
- Support dry-run validation
- Gracefully handle missing dependencies

## Instructions

- Write this software.
- Prefer to use a package over doing something manually. I have no problem pulling in dependencies. Check the `context7` MCP tool to find things you can use.
- Make sure you explore my Baseline repo (`/Users/dave/src/github.com/daveio/_baseline` ) and all repos underneath `/Users/dave/src` to get ideas of what can be set up, configured, and tweaked.
- Write comprehensive tests covering all functionality and edge cases.
- Include comprehensive documentation in [`README.md`](http://README.md) (for humans) and [`CLAUDE.md`](http://CLAUDE.md) (for AI).
- Create a template repository in `templates/` inside the project and use it.
- `git init` at the beginning and `commit` after each logical chunk. Commit often.
- Ask me if anything is unclear.
