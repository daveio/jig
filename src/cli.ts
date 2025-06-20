import { build } from "gluegun"

/**
 * Create the cli and kick it off
 */
// trunk-ignore-all(trunk-toolbox/todo): it's ignores all the way down
// @ts-ignore TS7006: TODO: give argv a proper type
async function run(argv) {
  // create a CLI runtime
  const cli = build()
    .brand("belt")
    .src(__dirname)
    .plugins("./node_modules", { matching: "belt-*", hidden: true })
    .help() // provides default for help, h, --help, -h
    .version() // provides default for version, v, --version, -v
    .create()
  // enable the following method if you'd like to skip loading one of these core extensions
  // this can improve performance if they're not necessary for your project:
  // .exclude(['meta', 'strings', 'print', 'filesystem', 'semver', 'system', 'prompt', 'http', 'template', 'patching', 'package-manager'])
  // and run it
  return await cli.run(argv)
}

module.exports = { run }
