# Task: scaffold a new CLI

Scaffold me a new Go CLI in the current directory.

- The existing code is a template.
- You should add libraries at their latest version at will.
  - I want to prefer pulling in libraries over implementing things individually.
  - I am not concerned about the size of our dependency graph.
  - Using libraries to reduce the amount of code we have to write is much more important than the size of our dependency graph.
  - Use `context7` to get information about any libraries you use. Don't trust your existing knowledge.
  - Use `toolbaseProxy:deepwiki*` to understand GitHub repositories.
    - If `context7` information conflicts with your existing knowledge, `context7` and `deepWiki` win.
  - Use `perplexity` and `tavily` to search the web, for example to find libraries.

## Libraries

### <https://github.com/alecthomas/kong> - Command Line Argument Parsing

Kong is a lightweight, declarative command-line parser that uses struct tags to define CLI behavior.

It handles:

- Command-line flags and arguments
- Sub-commands and command hierarchies
- Help text generation
- Validation of user input

Kong has gained popularity as a cleaner, more modern alternative to Cobra for CLI argument parsing.

### <https://github.com/companyinfo/conflex> - Configuration Management

Conflex is a configuration library.

Conflex is not on `context7` but you can use the `toolbaseProxy:deepwiki*` tools to get information.

- Easy Integration: Simple and intuitive API.
- Flexible Sources: Load from files, environment variables (with custom prefixes), Consul, and easily extend with custom sources.
- Format Agnostic: Supports JSON, YAML, and other formats via extensible codecs.
- Hierarchical Merging: Configurations from multiple sources are merged, with later sources overriding earlier ones.
- Struct Binding: Automatically map configuration data to Go structs.
- Built-in Validation: Validate configuration using struct methods, JSON Schemas, or custom functions.
- Dot Notation Access: Navigate nested configuration easily (e.g., config.GetString("database.host")).
- Type-Safe Retrieval: Get values as specific types (string, int, bool, etc.), with error-returning options for robust handling.
- Configuration Dumping: Save the effective configuration to files or other custom destinations.
- Clear Error Handling: Provides comprehensive error information for easier debugging.
- Thread-Safe: Safe for concurrent access and configuration loading in multi-goroutine applications.

### Charmbracelet Libraries - Terminal UI

The Charmbracelet ecosystem provides libraries for building rich terminal user interfaces:

- <https://github.com/charmbracelet/bubbletea>: A framework for building terminal applications based on The Elm Architecture
- <https://github.com/charmbracelet/bubbles>: A collection of UI components for Bubble Tea
- <https://github.com/charmbracelet/lipgloss>: A styling library for terminal applications
- <https://github.com/charmbracelet/huh>: A library for interactive forms and prompts

## Tools

Use `context7` to get documentation on these libraries and any others you want to pull in before you do anything else, so that you know their capabilities and any pitfalls.

Use `perplexity` and `tavily` to search the web, for example to find libraries.

Use `deepWiki` to understand GitHub repositories. If the wiki for a repository you want to understand has not been generated yet, start the generation process with email `dave@dave.io` if you can, and abort so that we can wait for it to be generated.

Don't just trust your inbuilt knowledge. Use the tools.

## Structure

Source should live within a `src/` directory.

## Arguments

### Invocation

We want to use a `belt [command] [subcommand?] [...subcommands?] {args}` structure.

### Shared Arguments

Arguments which are available in all commands (but may be ignored):

- `-h` / `--help` : Help: Give usage message and exit.
- `-v` / `--version` : Version: Show program version and exit.
- `-a` / `--all` : All: show all information or operate on all arguments.
- `-l` / `--list` : List: list files or arguments without taking other action.
- `-i` / `--input` : Read input from filename.
- `-o` / `--output` : Write output to filename.
- `-q` / `--quiet` : Quiet: less output to stdout.
- `-s` / `--silent` : Silent: No output to stdout.
- `-r` / `--recursive` : Recursive: Operate recursively (down directory tree).
- `-v` / `--verbose` : Verbose: output additional information to stdout or stderr.
- `-z` / `--compress` : Compress: apply zstd compression.
- `-f` / `--force` : Force: force overwrite or other destructive operation.
- `-p` / `--pipe` : Output structured data as JSON for use in a pipe.

We will consume these arguments in the command. Commands may add their own parameters, but they will never clash with the shared arguments. Add any you think are missing.

### Commands

Create a couple of example commands, some trivial tasks. Set the project up so that commands are in their own file.

`belt command args`: `src/commands/command/main.go`
`belt command subcommand args`: `src/commands/command/subcommand/main.go`

And so on. This will allow us to have multiple source files for each command as each command and subcommand (and sub-subcommand, etc) has its own directory.

### Helpers

Set up useful helpers for things like configuration and pretty output with the `charmbracelet` libraries, as well as any other shared infrastructure which might prove useful.

## Important things to keep in mind

- We want to use modern technologies, and libraries with active development.
  - An example of a library we decided not to use is `koanf`, because it is 6 years old.
  - There should be commit activity within the last 6 months for any libraries we use.
- Write idiomatic Go.

## Reminder: dependencies are good

- I am not concerned about the size of the dependency graph, and I am eager to use libraries instead of implementing things from scratch.
- That's true for the whole project, so bear it in mind as you're scaffolding things.
- Add packages freely, just make sure you add them at their latest version, and that they have a commit in the last 6 months.
  - Remember, `context7` and `deepWiki` can help you learn about a library.
- The date is 21 June, 2025.
