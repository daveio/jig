#!/usr/bin/env bun

import { Command } from "commander"
import { join } from "path"
import { ensureBunRuntime, findProjectRoot } from "./lib/utils.ts"
import { createOutput, setGlobalOutput } from "./lib/output.ts"
import { addSharedOptions, loadCommands, registerBeltCommand, setupCommandErrorHandling } from "./lib/command.ts"
import type { OutputLevel } from "./lib/types.ts"

async function main() {
  ensureBunRuntime()

  const program = new Command()

  program.name("belt").description("belt CLI - A Commander-based TypeScript CLI toolkit").version("0.0.1")

  addSharedOptions(program)

  const initialOutput = createOutput({
    level: "normal",
    pipe: false,
    noColor: process.env.NO_COLOR === "1"
  })
  setGlobalOutput(initialOutput)

  setupCommandErrorHandling(program)

  const projectRoot = findProjectRoot()
  const commandsDir = projectRoot ? join(projectRoot, "src", "commands") : join(import.meta.dir, "commands")

  try {
    const commands = await loadCommands(commandsDir)

    for (const command of commands) {
      registerBeltCommand(program, command)
    }

    if (commands.length === 0) {
      initialOutput.warn("No commands found in commands directory")
    } else {
      initialOutput.debug(`Loaded ${commands.length} command${commands.length === 1 ? "" : "s"}`)
    }
  } catch (error) {
    initialOutput.debug(`Failed to load commands: ${error}`)
  }

  program.hook("preAction", (thisCommand) => {
    const opts = thisCommand.opts()

    const level: OutputLevel = opts.silent ? "silent" : opts.quiet ? "quiet" : opts.verbose ? "verbose" : "normal"

    initialOutput.updateConfig({
      level,
      pipe: opts.pipe || false,
      noColor: process.env.NO_COLOR === "1" || opts.noColor
    })
  })

  program.addHelpText(
    "after",
    `

Examples:
  belt hello world          # Run hello command with 'world' argument
  belt list --recursive     # List files recursively
  belt --help               # Show this help message
  belt hello --help         # Show help for the hello command

For more information, visit: https://github.com/daveio/belt`
  )

  if (process.argv.length <= 2) {
    program.help()
  }

  await program.parseAsync(process.argv)
}

if (import.meta.main) {
  main().catch((error) => {
    console.error("Fatal error:", error)
    process.exit(1)
  })
}
