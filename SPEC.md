# `jig`

## About

`jig` is a tool to manage various aspects of my development environment.

It auto-detects languages, package managers, and configurations to tweak.

**Language:** Rust

**Template engine:** Ask the `context7` MCP tool. Find something flexible and sensible.

**Git support:** Use a `git` library instead of shelling out. Ask the `context7` MCP tool for one.

## Template System

_[imagine a fox juggling template files, with shared components glowing in the center]_

### Directory Structure

We've organized templates with a shared component system to reduce duplication:

```
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

### `jig bump [repository]`

- Bumps versions to latest, in various package managers and configuration files.
- Explore all the repositories in `/Users/dave/src` to find package managers and configuration files to set up.
- Python should always use `uv` and migrate to it if anything else is used (for example `poetry`).
  - Use `PEP-518` keys in `pyproject.toml` . See [https://peps.python.org/pep-0518](https://peps.python.org/pep-0518/)
- `jig bump` without `[repository]` updates the current repository.
- Include GitHub Actions. Fetch the latest commit for the repository on `main` (and `master` if `main` doesn't exist) and update them to use it, even if they're configured for a tag.

## Future Enhancements

Potential future improvements to the template system:

1. Additional component types beyond gitignore and workflows
2. More granular control over component inclusion
3. Version-specific components for different language versions

## Instructions

- Write this software.
- Prefer to use a package over doing something manually. I have no problem pulling in dependencies. Check the `context7` MCP tool to find things you can use.
- Make sure you explore my Baseline repo (`/Users/dave/src/github.com/daveio/_baseline` ) and all repos underneath `/Users/dave/src` to get ideas of what can be set up, configured, and tweaked.
- Write tests.
- Include comprehensive documentation in [`README.md`](http://README.md) (for humans) and [`CLAUDE.md`](http://CLAUDE.md) (for AI).
- Create a template repository in `templates/` inside the project and use it.
- `git init` at the beginning and `commit` after each logical chunk. Commit often.
- Ask me if anything is unclear.
