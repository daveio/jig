import { Command } from "commander"
import type { BeltCommand, CommandContext, SharedOptions, BeltError } from "./types.ts"
import { getGlobalOutput } from "./output.ts"

export function createBeltError(message: string, code?: string, exitCode?: number): BeltError {
  const error = new Error(message) as BeltError
  error.code = code
  error.exitCode = exitCode || 1
  return error
}

export function addSharedOptions(command: Command): Command {
  return command
    .option("-a, --all", "show all information or operate on all arguments")
    .option("-l, --list", "list files or arguments without taking other action")
    .option("-i, --input <filename>", "read input from filename")
    .option("-o, --output <filename>", "write output to filename")
    .option("-q, --quiet", "quiet: less output to stdout")
    .option("-s, --silent", "silent: no output to stdout")
    .option("-r, --recursive", "recursive: operate recursively (down directory tree)")
    .option("-v, --verbose", "verbose: output additional information to stdout or stderr")
    .option("-z, --compress", "compress: apply zstd compression")
    .option("-f, --force", "force: force overwrite or other destructive operation")
    .option("-p, --pipe", "output structured data as JSON for use in a pipe")
}

export function parseSharedOptions(command: Command): SharedOptions {
  const opts = command.opts()

  return {
    help: opts.help,
    version: opts.version,
    all: opts.all,
    list: opts.list,
    input: opts.input,
    output: opts.output,
    quiet: opts.quiet,
    silent: opts.silent,
    recursive: opts.recursive,
    verbose: opts.verbose,
    compress: opts.compress,
    force: opts.force,
    pipe: opts.pipe
  }
}

export function createCommandContext(command: Command, args: string[]): CommandContext {
  return {
    options: parseSharedOptions(command),
    command,
    args
  }
}

export function registerBeltCommand(program: Command, beltCommand: BeltCommand): Command {
  let command = program.command(beltCommand.name)

  if (beltCommand.description) {
    command.description(beltCommand.description)
  }

  if (beltCommand.arguments) {
    for (const arg of beltCommand.arguments) {
      const argStr = arg.required
        ? arg.variadic
          ? `<${arg.name}...>`
          : `<${arg.name}>`
        : arg.variadic
          ? `[${arg.name}...]`
          : `[${arg.name}]`

      command.argument(argStr, arg.description)
    }
  }

  command = addSharedOptions(command)

  if (beltCommand.options) {
    for (const opt of beltCommand.options) {
      if (opt.choices) {
        command.option(opt.flags, opt.description, opt.defaultValue)
        // Add validation for choices if needed
      } else {
        command.option(opt.flags, opt.description, opt.defaultValue)
      }
    }
  }

  command.action(async (...args) => {
    try {
      const lastArg = args[args.length - 1]
      const cmd = lastArg as Command
      const commandArgs = args.slice(0, -2) as string[]

      const context = createCommandContext(cmd, commandArgs)

      // Configure output based on options
      const output = getGlobalOutput()
      const level = context.options.silent
        ? "silent"
        : context.options.quiet
          ? "quiet"
          : context.options.verbose
            ? "verbose"
            : "normal"

      output.updateConfig({
        level,
        pipe: context.options.pipe || false,
        noColor: process.env.NO_COLOR === "1"
      })

      await beltCommand.action(context)
    } catch (error) {
      const output = getGlobalOutput()

      if (error instanceof Error) {
        const beltError = error as BeltError
        output.error(beltError.message, "quiet")

        if (beltError.code) {
          output.debug(`Error code: ${beltError.code}`)
        }

        process.exit(beltError.exitCode || 1)
      } else {
        output.error("An unexpected error occurred", "quiet")
        process.exit(1)
      }
    }
  })

  return command
}

export async function loadCommands(commandsDir: string): Promise<BeltCommand[]> {
  const commands: BeltCommand[] = []

  try {
    const { readdirSync } = await import("fs")
    const files = readdirSync(commandsDir)

    for (const file of files) {
      if (file.endsWith(".ts") || file.endsWith(".js")) {
        try {
          const commandPath = `${commandsDir}/${file}`
          const module = await import(commandPath)

          if (module.default && typeof module.default === "object") {
            commands.push(module.default as BeltCommand)
          } else if (module.command && typeof module.command === "object") {
            commands.push(module.command as BeltCommand)
          }
        } catch (error) {
          const output = getGlobalOutput()
          output.warn(`Failed to load command from ${file}: ${error}`, "verbose")
        }
      }
    }
  } catch (error) {
    const output = getGlobalOutput()
    output.debug(`Commands directory not found or not readable: ${commandsDir}`)
  }

  return commands
}

export function setupCommandErrorHandling(program: Command) {
  program.exitOverride((err) => {
    const output = getGlobalOutput()

    if (err.code === "commander.unknownCommand") {
      output.error(`Unknown command: ${err.message}`)
      output.info("Run 'belt --help' for available commands")
    } else if (err.code === "commander.unknownOption") {
      output.error(`Unknown option: ${err.message}`)
    } else if (err.code === "commander.missingArgument") {
      output.error(`Missing argument: ${err.message}`)
    } else if (err.code === "commander.optionMissingArgument") {
      output.error(`Option missing argument: ${err.message}`)
    } else {
      output.error(err.message)
    }

    process.exit(err.exitCode || 1)
  })
}
