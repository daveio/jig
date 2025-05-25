# Hubbit - Git Repository and Binary Manager

## Overview

Create a command-line application that helps developers manage Git repositories (primarily from GitHub) and release binaries. The application should provide a streamlined workflow for cloning repositories and potentially downloading binary releases from GitHub.

## Core Functionality

### Repository Management

The application should:

1. Parse repository identifiers in various formats:

- Simple repository name (e.g., `repo`)
- Username/repository (e.g., `username/repo`)
- Full URLs (e.g., `https://github.com/username/repo`)
- SSH URLs (e.g., `git@github.com:username/repo.git`)

2. Intelligently handle repository cloning:

- When given just a repository name, use the user's configured GitHub username
- Support both HTTPS and SSH protocols for cloning
- Select the appropriate protocol based on configuration or URL format
- Clone to appropriate locations based on a configurable directory structure

3. Provide user-friendly output:

- Display clear progress information during operations
- Handle errors gracefully with helpful messages
- Support verbose mode for detailed output

### Configuration Management

The application should:

1. Maintain user configuration in a standard location (`~/.config/hubbit/config.yaml`)
2. Support these configuration options:

- GitHub username
- GitHub token (for authenticated operations)
- Preferred protocol (SSH or HTTPS)
- Default clone location
- Whether to use direct Git library integration or CLI commands

### Future Binary Management (Planned Feature)

The application should eventually:

1. List available releases for a repository
2. Download specific binaries from releases
3. Manage downloaded binaries (versioning, updates, etc.)

## User Experience

The application should be invoked as a command-line tool with a simple, intuitive syntax:

```bash
hubbit clone repo
hubbit clone username/repo
hubbit clone https://hostname.com/repo
hubbit clone git@hostname.com:/repo
```

The application should intelligently parse these inputs and perform the appropriate action.

The application also implements `hubbit binary` to download, update, and generally manage GitHub release binaries. It should store data about installed binaries in `~/.config/hubbit/binaries.yaml` to support updating.

It should handle binaries in compressed and archive formats as well as plain binaries.

- `hubbit binary get [repo]` - Download the latest binary for a repository, with intelligence about the platform we are using.
- `hubbit binary update [repo]` - Check for updates for the binary for a repository, and update if available.
- `hubbit binary update` - Check for updates for all binaries and update.

The repo specifier will be the same as for `hubbit clone`.

## Application Flow

1. User invokes the application with arguments
2. Application parses the command and repository identifier
3. Application determines:

- What repository is being referenced
- What protocol to use (SSH/HTTPS)
- Where to clone the repository

4. Application executes the operation and provides feedback
5. Application handles success/failure gracefully

## Configuration File Example

The application should support a configuration file like:

```yaml
main:
  external_git: false # Whether to use a Git library or the git CLI

github:
  username: myusername # GitHub username
  token: mytoken # GitHub authentication token
  protocol: ssh # Default protocol (ssh or https)
  clone_directory: ~/dev # Where to clone repositories
```

## Error Handling

The application should provide helpful error messages for common issues:

- Missing configuration
- Invalid repository format
- Network issues
- Authentication failures
- Permission problems

## Technical Requirements

The application should:

- Be implemented as a standalone command-line tool
- Follow platform conventions for configuration storage
- Use appropriate libraries for Git operations
- Have a clean, modular architecture for future expansion
- Include comprehensive error handling
- Be well-tested with automated tests
- Support common platforms (Linux, macOS, Windows)

## Additional Infrastructure

You should also create GitHub Actions workflows:

- `build.yaml` - Build the application and run tests
- `check.yaml` - Check the application for linting, formatting, and security issues
- `release.yaml` - Release the application to GitHub, run on releases

## Development Standards

The application should:

- Follow standard project structure for the implementation language
- Have clear documentation both in code and for users
- Use semantic versioning
- Include CI/CD integration
- Follow best practices for code quality and security

## Future Enhancements

The roadmap could include:

- Managing multiple GitHub accounts
- Supporting other Git platforms (GitLab, Bitbucket)
- Advanced repository organization features
- Binary version management
- Integration with package managers
- Repository statistics and insights
