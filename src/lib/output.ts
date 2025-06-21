import chalk from "chalk"
import boxen from "boxen"
import ora, { type Ora } from "ora"
import cliProgress from "cli-progress"
import pc from "picocolors"
import type { OutputConfig, OutputLevel, SpinnerOptions, ProgressOptions } from "./types.ts"

export class OutputManager {
  private config: OutputConfig
  private spinner: Ora | null = null
  private progressBar: cliProgress.SingleBar | null = null

  constructor(config: OutputConfig) {
    this.config = config
  }

  updateConfig(config: Partial<OutputConfig>) {
    this.config = { ...this.config, ...config }
  }

  private shouldOutput(level: OutputLevel): boolean {
    const levels: Record<OutputLevel, number> = {
      silent: 0,
      quiet: 1,
      normal: 2,
      verbose: 3
    }
    return levels[this.config.level] >= levels[level]
  }

  private formatForPipe(data: any): string {
    return JSON.stringify(data, null, this.config.pipe ? 0 : 2)
  }

  log(message: string, level: OutputLevel = "normal") {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "log", message, level }))
      return
    }

    if (!this.shouldOutput(level)) return

    console.log(message)
  }

  info(message: string, level: OutputLevel = "normal") {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "info", message, level }))
      return
    }

    if (!this.shouldOutput(level)) return

    const prefix = this.config.noColor ? "[INFO]" : chalk.blue("[INFO]")
    console.log(`${prefix} ${message}`)
  }

  success(message: string, level: OutputLevel = "normal") {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "success", message, level }))
      return
    }

    if (!this.shouldOutput(level)) return

    const prefix = this.config.noColor ? "[✓]" : chalk.green("[✓]")
    console.log(`${prefix} ${message}`)
  }

  warn(message: string, level: OutputLevel = "quiet") {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "warning", message, level }))
      return
    }

    if (!this.shouldOutput(level)) return

    const prefix = this.config.noColor ? "[WARN]" : chalk.yellow("[WARN]")
    console.warn(`${prefix} ${message}`)
  }

  error(message: string, level: OutputLevel = "quiet") {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "error", message, level }))
      return
    }

    if (!this.shouldOutput(level)) return

    const prefix = this.config.noColor ? "[ERROR]" : chalk.red("[ERROR]")
    console.error(`${prefix} ${message}`)
  }

  debug(message: string) {
    this.log(this.config.noColor ? `[DEBUG] ${message}` : chalk.gray(`[DEBUG] ${message}`), "verbose")
  }

  box(message: string, title?: string) {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "box", message, title }))
      return
    }

    if (!this.shouldOutput("normal")) return

    const options: {
      padding: number
      margin: number
      borderStyle: "round"
      borderColor?: string
      title?: string
    } = {
      padding: 1,
      margin: 1,
      borderStyle: "round",
      borderColor: this.config.noColor ? undefined : "cyan"
    }

    if (title) {
      options.title = title
    }

    console.log(boxen(message, options))
  }

  table(data: Record<string, any>[], headers?: string[]) {
    if (this.config.pipe) {
      console.log(this.formatForPipe({ type: "table", data, headers }))
      return
    }

    if (!this.shouldOutput("normal") || data.length === 0) return

    const keys = headers || Object.keys(data[0] || {})
    if (!keys || keys.length === 0) return

    const rows = [keys, ...data.map((row) => keys.map((key) => String(row[key] || "")))]

    const colWidths = keys.map((_, i) => Math.max(...rows.map((row) => String(row?.[i] || "").length)))

    const formatRow = (row: string[], isHeader = false) => {
      const formatted = row.map((cell, i) => cell.padEnd(colWidths[i] || 0)).join(" | ")
      return isHeader && !this.config.noColor ? chalk.bold(formatted) : formatted
    }

    const firstRow = rows[0]
    if (firstRow) {
      console.log(formatRow(firstRow, true))
      console.log(colWidths.map((w) => "-".repeat(w)).join("-|-"))
      rows.slice(1).forEach((row) => console.log(formatRow(row)))
    }
  }

  startSpinner(options: SpinnerOptions) {
    if (this.config.pipe || !this.shouldOutput("normal")) return

    this.stopSpinner()
    this.spinner = ora({
      text: options.text,
      color: options.color || "cyan"
    }).start()
  }

  updateSpinner(text: string) {
    if (this.spinner) {
      this.spinner.text = text
    }
  }

  stopSpinner(success?: boolean, finalMessage?: string) {
    if (!this.spinner) return

    if (success !== undefined) {
      if (success) {
        this.spinner.succeed(finalMessage)
      } else {
        this.spinner.fail(finalMessage)
      }
    } else {
      this.spinner.stop()
      if (finalMessage) {
        this.log(finalMessage)
      }
    }

    this.spinner = null
  }

  startProgress(options: ProgressOptions) {
    if (this.config.pipe || !this.shouldOutput("normal")) return

    this.stopProgress()

    this.progressBar = new cliProgress.SingleBar({
      format: options.format || `${options.title || "Progress"} |{bar}| {percentage}% | {value}/{total}`,
      barCompleteChar: "█",
      barIncompleteChar: "░",
      hideCursor: true
    })

    this.progressBar.start(options.total, 0)
  }

  updateProgress(value: number) {
    if (this.progressBar) {
      this.progressBar.update(value)
    }
  }

  stopProgress() {
    if (this.progressBar) {
      this.progressBar.stop()
      this.progressBar = null
    }
  }

  br() {
    if (this.config.pipe) return
    if (this.shouldOutput("normal")) {
      console.log()
    }
  }

  json(data: any) {
    console.log(this.formatForPipe(data))
  }
}

let globalOutput: OutputManager | null = null

export function createOutput(config: OutputConfig): OutputManager {
  return new OutputManager(config)
}

export function setGlobalOutput(output: OutputManager) {
  globalOutput = output
}

export function getGlobalOutput(): OutputManager {
  if (!globalOutput) {
    throw new Error("Global output manager not initialized")
  }
  return globalOutput
}

export const colors = {
  red: (text: string) => pc.red(text),
  green: (text: string) => pc.green(text),
  yellow: (text: string) => pc.yellow(text),
  blue: (text: string) => pc.blue(text),
  magenta: (text: string) => pc.magenta(text),
  cyan: (text: string) => pc.cyan(text),
  white: (text: string) => pc.white(text),
  gray: (text: string) => pc.gray(text),
  bold: (text: string) => pc.bold(text),
  dim: (text: string) => pc.dim(text)
}
