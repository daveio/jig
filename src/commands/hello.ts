import type { BeltCommand, CommandContext } from "../lib/types.ts"
import { getGlobalOutput } from "../lib/output.ts"

const hello: BeltCommand = {
  name: "hello",
  description: "Greet someone with a friendly message",
  arguments: [
    {
      name: "name",
      description: "Name of the person to greet",
      required: false
    }
  ],
  options: [
    {
      flags: "--formal",
      description: "Use formal greeting"
    },
    {
      flags: "--language <lang>",
      description: "Language for greeting",
      defaultValue: "en",
      choices: ["en", "es", "fr", "de", "it"]
    },
    {
      flags: "--repeat <count>",
      description: "Number of times to repeat the greeting",
      defaultValue: "1"
    }
  ],
  action: async (context: CommandContext) => {
    const output = getGlobalOutput()
    const { args, options } = context

    const name = args[0] || "World"
    const commandOpts = context.command.opts()
    const formal = commandOpts.formal || false
    const language = commandOpts.language || "en"
    const repeat = parseInt(commandOpts.repeat || "1", 10)

    if (isNaN(repeat) || repeat < 1) {
      throw new Error("Repeat count must be a positive number")
    }

    if (repeat > 10) {
      if (!options.force) {
        throw new Error("Repeat count too high (use --force to override)")
      }
      output.warn("That's a lot of greetings! Hope you have time...")
    }

    const greetings: Record<string, { casual: string; formal: string }> = {
      en: { casual: "Hello", formal: "Good day" },
      es: { casual: "Hola", formal: "Buenos días" },
      fr: { casual: "Salut", formal: "Bonjour" },
      de: { casual: "Hallo", formal: "Guten Tag" },
      it: { casual: "Ciao", formal: "Buongiorno" }
    }

    const greeting = greetings[language]
    if (!greeting) {
      throw new Error(`Unsupported language: ${language}`)
    }

    const greetingText = formal ? greeting.formal : greeting.casual
    const message = `${greetingText}, ${name}!`

    if (options.pipe) {
      output.json({
        greeting: message,
        name,
        language,
        formal,
        repeat
      })
      return
    }

    if (options.verbose) {
      output.info(`Generating ${repeat} greeting${repeat > 1 ? "s" : ""} in ${language}`)
      if (formal) {
        output.info("Using formal greeting style")
      }
    }

    for (let i = 0; i < repeat; i++) {
      if (options.all || options.verbose) {
        output.success(`[${i + 1}/${repeat}] ${message}`)
      } else {
        output.success(message)
      }

      if (i < repeat - 1 && repeat > 1) {
        output.log("")
      }
    }

    if (options.verbose) {
      output.info("Greeting complete! 👋")
    }
  }
}

export default hello
