import type { Command } from "commander"

export interface SharedOptions {
  help?: boolean
  version?: boolean
  all?: boolean
  list?: boolean
  input?: string
  output?: string
  quiet?: boolean
  silent?: boolean
  recursive?: boolean
  verbose?: boolean
  compress?: boolean
  force?: boolean
  pipe?: boolean
}

export interface CommandContext {
  options: SharedOptions
  command: Command
  args: string[]
}

export interface BeltCommand {
  name: string
  description: string
  arguments?: CommandArgument[]
  options?: CommandOption[]
  action: (context: CommandContext) => Promise<void> | void
}

export interface CommandArgument {
  name: string
  description: string
  required?: boolean
  variadic?: boolean
}

export interface CommandOption {
  flags: string
  description: string
  defaultValue?: any
  choices?: string[]
}

export type OutputLevel = "silent" | "quiet" | "normal" | "verbose"

export interface OutputConfig {
  level: OutputLevel
  pipe: boolean
  noColor?: boolean
}

export interface SpinnerOptions {
  text: string
  color?: "red" | "green" | "yellow" | "blue" | "magenta" | "cyan" | "white"
}

export interface ProgressOptions {
  total: number
  title?: string
  format?: string
}

export interface BeltError extends Error {
  code?: string
  exitCode?: number
}

export interface RuntimeInfo {
  isNode: boolean
  isBun: boolean
  version: string
  platform: string
}
