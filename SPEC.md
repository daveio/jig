# `jig`

## About

`jig` is a tool to manage various aspects of my development environment.

It auto-detects languages, package managers, and configurations to tweak.

**Language:** Rust

**Template engine:** Ask the `context7` MCP tool. Find something flexible and sensible.

**Git support:** Use a `git` library instead of shelling out. Ask the `context7` MCP tool for one.

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

## Instructions

- Write this software.
- Prefer to use a package over doing something manually. I have no problem pulling in dependencies. Check the `context7` MCP tool to find things you can use.
- Make sure you explore my Baseline repo (`/Users/dave/src/github.com/daveio/_baseline` ) and all repos underneath `/Users/dave/src` to get ideas of what can be set up, configured, and tweaked.
- Write tests.
- Include comprehensive documentation in [`README.md`](http://README.md) (for humans) and [`CLAUDE.md`](http://CLAUDE.md) (for AI).
- Create a template repository in `templates/` inside the project and use it.
- `git init` at the beginning and `commit` after each logical chunk. Commit often.
- Ask me if anything is unclear.
