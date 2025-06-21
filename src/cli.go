package main

import (
	"fmt"
	"os"

	"github.com/alecthomas/kong"
	"github.com/daveio/belt/src/commands/audio"
	"github.com/daveio/belt/src/commands/crypt"
	"github.com/daveio/belt/src/commands/dns"
	"github.com/daveio/belt/src/commands/domain"
	initcmd "github.com/daveio/belt/src/commands/init"
	"github.com/daveio/belt/src/commands/tls"
	"github.com/daveio/belt/src/config"
	"github.com/daveio/belt/src/internal/output"
	"github.com/daveio/belt/src/internal/types"
	"github.com/daveio/belt/src/internal/version"
)

// CLI represents the command line interface structure.
type CLI struct {
	// Global flags available to all commands
	Help      bool   `help:"Show help information and exit."                             short:"h"`
	Version   bool   `help:"Show program version and exit."                              short:"V"`
	All       bool   `help:"Show all information or operate on all arguments."           short:"a"`
	Input     string `help:"Read input from filename."                                   short:"i" type:"path"`
	Output    string `help:"Write output to filename."                                   short:"o" type:"path"`
	Quiet     bool   `help:"Quiet: less output to stdout."                               short:"q"`
	Silent    bool   `help:"Silent: No output to stdout."                                short:"s"`
	Recursive bool   `help:"Recursive: Operate recursively (down directory tree)."       short:"r"`
	Verbose   bool   `help:"Verbose: output additional information to stdout or stderr." short:"v"`
	Compress  bool   `help:"Compress: apply zstd compression."                           short:"z"`
	Force     bool   `help:"Force: force overwrite or other destructive operation."      short:"f"`
	Pipe      bool   `help:"Output structured data as JSON for use in a pipe."           short:"p"`

	// Commands
	Init   initcmd.Cmd `cmd:"" help:"Initialize configuration file."`
	Audio  audio.Cmd   `cmd:"" help:"Audio file operations."`
	Crypt  crypt.Cmd   `cmd:"" help:"Cryptography operations."`
	DNS    dns.Cmd     `cmd:"" help:"DNS operations."`
	Domain domain.Cmd  `cmd:"" help:"Domain operations."`
	TLS    tls.Cmd     `cmd:"" help:"TLS certificate operations."`
}

// Run executes the CLI application.
func (cli *CLI) Run(ctx *types.Context) error {
	// Apply global flags to configuration
	if cli.Quiet {
		ctx.Config.Output.Quiet = true
	}

	if cli.Silent {
		ctx.Config.Output.Silent = true
	}

	if cli.Pipe {
		ctx.Config.Output.Format = "json"
		// Recreate output writer with JSON format
		ctx.Output = output.NewStdout(output.FormatJSON)
	}

	return nil
}

func main() {
	// Check for version flag before doing anything else
	if version.CheckVersionFlag(os.Args) {
		return
	}

	// Load configuration
	cfg, err := config.Load()
	if err != nil {
		_, err := fmt.Fprintf(os.Stderr, "Error loading configuration: %v\n", err)
		if err != nil {
			return
		}
		os.Exit(1)
	}

	// Check if this is the first run (no config file)
	if !config.ConfigExists() && len(os.Args) > 1 && os.Args[1] != "init" {
		_, err := fmt.Fprintf(
			os.Stderr,
			"No configuration file found. Run 'belt init' to create one.\n",
		)
		if err != nil {
			return
		}
		os.Exit(1)
	}

	// Create an output writer
	outputFormat := output.FormatAuto
	if cfg.Output.Format == "json" {
		outputFormat = output.FormatJSON
	}

	out := output.NewStdout(outputFormat)

	// Create a shared context
	ctx := &types.Context{
		Config: cfg,
		Output: out,
	}

	// Parse command line
	var cli CLI
	kongCtx := kong.Parse(&cli,
		kong.Name("belt"),
		kong.Description("A modular CLI toolbelt for cryptography, DNS, audio, and more."),
		kong.Vars{
			"version": version.VersionString(),
		},
		kong.UsageOnError(),
		kong.ConfigureHelp(kong.HelpOptions{
			Compact: true,
		}),
		kong.Bind(ctx),
	)

	// Execute the command
	err = kongCtx.Run()
	if err != nil {
		out.PrintError(fmt.Sprintf("Command failed: %v", err))
		os.Exit(1)
	}
}
