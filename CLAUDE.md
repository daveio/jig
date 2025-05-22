# jig - Documentation for AI Assistants

This document is intended for AI assistants (like Claude) to understand how to use and help develop the `jig` tool.

## Overview

`jig` is a tool for managing development environments. It helps with:

1. Creating new repositories with templates and best practices
2. Updating repositories from templates
3. Configuring AI tools with best practices
4. Bumping dependencies to the latest versions
5. Managing Dependabot configuration for automated dependency updates

## Architecture

The codebase is organized into the following modules:

- `cli`: Command-line interface handling
- `commands`: Implementation of the various commands
  - `new`: Creating new repositories
  - `update`: Updating repositories from templates
  - `ai`: Configuring AI tools
  - `bump`: Bumping dependency versions
  - `dependabot`: Managing Dependabot configuration
- `template`: Template handling and language detection
- `git`: Git operations
- `package_manager`: Package manager operations
- `ai`: AI tool configuration
- `config`: Configuration handling
- `dependabot`: Dependabot configuration detection and management
- `utils`: Utility functions

## CLI Options

### Global Options

All commands support these global options:

- `--dry-run, -d`: Run in dry-run mode (only explain what would be changed)
- `--info, -i`: Show information about changes without making them
- `--verbose, -v`: Show detailed information during execution
- `--ai, -a`: Format output for AI consumption (requires --info)
- `--output, -o <PATH>`: Redirect output to a file

### Commands

#### `jig new <language>`

Create a new repository with the specified language template.

#### `jig update [repository]`

Update repository from templates. Uses current directory if repository not specified.

#### `jig ai [tool]`

Configure AI tools. If no tool specified, configures all supported tools:

- claude-desktop
- cursor
- zed
- goose

#### `jig bump [repository] [--ecosystem <ecosystem>]`

Bump dependency versions. Supports ecosystem-specific updates:

- `node`: Update Node.js/npm packages
- `python`: Update Python packages
- `rust`: Update Rust packages
- `ruby`: Update Ruby packages
- `java`: Update Java packages
- `go`: Update Go packages
- `actions`: Update GitHub Actions workflows only

Additional options:

- `--cached`: Use cached versions only (don't check for updates online)

#### `jig dependabot [repository]`

Manage Dependabot configuration. Detects package ecosystems and creates/updates `.github/dependabot.yml`.

## Template System

Templates are stored in the `templates` directory, with a new structure that promotes reuse:

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
└── ... other languages
```

### Template Filename Requirements

When working with template files, never use template syntax (like `{{ variable }}`) directly in filenames. Instead:

1. Use static placeholder names like `PROJECT_NAME.gemspec.tera` in filenames
2. Place all template logic inside the file content where it belongs
3. The system will translate these placeholders based on the project context

Files with template syntax in filenames (e.g., `{{ project_name|lower|replace(from=" ", to="_") }}.gemspec.tera`) should be renamed to use placeholders (e.g., `PROJECT_NAME.gemspec.tera`).

The code handles the translation of these placeholders to appropriate filenames during template processing. This improves maintainability and makes the template system more robust.

### Shared Components

The `shared/` directory contains components that are used by multiple language templates:

1. **Gitignore patterns**:

   - `common.gitignore`: Patterns common to all projects (IDE files, logs, env vars)
   - `[language].gitignore`: Language-specific patterns

2. **GitHub workflows**:
   - `base.yml.tera`: Common workflow structure
   - `[language]_jobs.yml.tera`: Language-specific jobs

### Placeholder Files

In each language directory, placeholder files specify which shared components to use:

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

### Template Variables

The template system uses the following variables:

- `project_name`: The name of the project
- `language`: The programming language
- Workflow-specific variables:
  - `workflow_name`: Name of the workflow
  - `env`: Environment variables (optional)
  - `jobs_template`: Which jobs template to use

## Implementation Details

### Template Processing

When processing templates, `jig` now:

1. Reads placeholder files to determine which shared components to include
2. Combines shared components as specified
3. Processes the combined templates with Tera

For example, when processing a `gitignore` placeholder:

- Read the placeholder file to get a list of components
- Read each component from `shared/gitignore/`
- Concatenate the components in order
- Save the result as `.gitignore` in the output directory

### Git Operations

Git operations use the `git2` crate instead of shelling out to git. Key operations include:

- Initializing repositories
- Opening existing repositories
- Committing changes
- Getting the default branch

### Package Manager Detection

The tool auto-detects package managers in repositories:

- Python: pyproject.toml, requirements.txt, setup.py, poetry.lock
- JavaScript/TypeScript: package.json
- Rust: Cargo.toml

### Dependabot Configuration

The `dependabot` module (`src/dependabot/mod.rs`) provides:

- `detect_ecosystems()`: Scans a repository to detect all package ecosystems
- `find_dependabot_config()`: Locates existing dependabot configuration
- `update_dependabot_config()`: Creates or updates dependabot configuration

Supported ecosystems:

- npm (Node.js)
- pip (Python)
- cargo (Rust)
- bundler (Ruby)
- gomod (Go)
- maven/gradle/sbt (Java/Scala)
- composer (PHP)
- docker
- github-actions
- And more...

### AI Tool Configuration

AI tools are configured by copying configuration files from a baseline repository:

- Claude Desktop: mcp-claude-desktop.json
- Cursor: \_cursor directory and mcp-cursor.json
- Zed: .zed directory and mcp-zed.json
- Goose: mcp-goose.yaml

## Project Infrastructure

### Development Tools

The project uses several tools for development and quality assurance:

1. **mise**: Tool version management and task runner

   - Configured in `mise.toml`
   - Manages Rust toolchain version
   - Provides standardized tasks (build, test, fmt, lint, ci)

2. **trunk**: Multi-language linter and formatter

   - Configuration in `.trunk/trunk.yaml`
   - Handles code formatting and linting

3. **GitHub Actions**: CI/CD pipelines

   - `ci.yaml`: Main CI workflow (build, test)
   - `codacy.yaml`: Code quality analysis
   - `devskim.yaml`: Security scanning
   - `trunk.yaml`: Linting checks

4. **Dependabot**: Automated dependency updates
   - Configuration in `.github/dependabot.yml`
   - Monitors Rust dependencies

## Extending the Tool

### Adding a New Language

To add support for a new language:

1. Create language-specific shared components in `shared/`
2. Create a new directory in `templates/` for the language
3. Add placeholder files referencing the shared components
4. Add language-specific template files
5. Update the language detection in `template/language.rs`

### Adding Shared Components

To add a new shared component:

1. Identify the component type (gitignore, workflow, etc.)
2. Add it to the appropriate directory in `shared/`
3. Update placeholder files in language directories to reference it

### Adding a New Package Manager

To add support for a new package manager:

1. Create a new module in `package_manager/`
2. Implement the `bump_versions` function
3. Add the package manager to the `bump_all_versions` function in `package_manager/mod.rs`

### Adding a New AI Tool

To add support for a new AI tool:

1. Add the tool to the `AiTool` enum in `ai/mod.rs`
2. Implement the `from_str`, `name`, and `config_path` functions
3. Add the tool to the `configure_tool` function

### Adding Dependabot Support

The dependabot command automatically detects package ecosystems by looking for specific files:

- `package.json` → npm
- `Cargo.toml` → cargo
- `pyproject.toml` or `requirements.txt` → pip
- And many more...

The detection logic is in `src/dependabot/mod.rs::detect_ecosystems()`.

## Common Tasks

### Debugging

The tool uses the `env_logger` crate for logging. Set the `RUST_LOG` environment variable to control logging levels:

```bash
RUST_LOG=debug jig new rust
```

### Testing

Run tests with:

```bash
cargo test
# or using mise
mise test
```

### Building

Build the tool with:

```bash
cargo build --release
# or using mise
mise build
```

### Running CI Locally

To run the full CI suite locally:

```bash
mise ci
```

This runs formatting, linting, and tests in sequence.

### YAML Handling

The tool uses `yaml-rust2` for YAML parsing instead of the deprecated `serde_yaml`. This is a maintained fork of the original `yaml-rust` that is fully compliant with the YAML 1.2 specification.

For working with YAML files, use the utility functions in `utils/yaml.rs`:

```rust
// Parse a YAML file
let yaml = utils::yaml::parse_file(&path)?;

// Parse a YAML string
let yaml = utils::yaml::parse_string(&content)?;

// Convert a Yaml value to a string
let yaml_str = utils::yaml::to_string(&yaml)?;

// Write a Yaml value to a file
utils::yaml::write_file(&yaml, &path)?;
```

Note that `yaml-rust2` uses its own `Yaml` type for representing YAML data, which is different from Serde's approach. When you need to convert between `yaml-rust2::Yaml` and your Serde structures, you'll need to implement custom conversion logic.
