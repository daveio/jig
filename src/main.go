package main

import (
	"fmt"
	"os"

	"github.com/alecthomas/kong"
	"github.com/daveio/belt/src/commands/format"
	"github.com/daveio/belt/src/commands/hello"
	"github.com/daveio/belt/src/commands/list"
	"github.com/daveio/belt/src/config"
	"github.com/daveio/belt/src/internal/output"
	"github.com/daveio/belt/src/internal/types"
)

// CLI represents the command line interface structure
type CLI struct {
	// Global flags available to all commands
	Help      bool   `short:"h" help:"Show help information and exit."`
	Version   bool   `short:"v" help:"Show program version and exit."`
	All       bool   `short:"a" help:"Show all information or operate on all arguments."`
	Input     string `short:"i" help:"Read input from filename." type:"path"`
	Output    string `short:"o" help:"Write output to filename." type:"path"`
	Quiet     bool   `short:"q" help:"Quiet: less output to stdout."`
	Silent    bool   `short:"s" help:"Silent: No output to stdout."`
	Recursive bool   `short:"r" help:"Recursive: Operate recursively (down directory tree)."`
	Verbose   bool   `help:"Verbose: output additional information to stdout or stderr."`
	Compress  bool   `short:"z" help:"Compress: apply zstd compression."`
	Force     bool   `short:"f" help:"Force: force overwrite or other destructive operation."`
	Pipe      bool   `short:"p" help:"Output structured data as JSON for use in a pipe."`

	// Commands
	Hello  hello.Cmd  `cmd:"" help:"Say hello to someone."`
	List   list.Cmd   `cmd:"" help:"List files and directories."`
	Format format.Cmd `cmd:"" help:"Format data in various ways."`
}

// Run executes the CLI application
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

var (
	version = "1.0.0"
	commit  = "dev"
	date    = "unknown"
)

func main() {
	// Load configuration
	cfg, err := config.Load()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error loading configuration: %v\n", err)
		os.Exit(1)
	}

	// Create output writer
	outputFormat := output.FormatAuto
	if cfg.Output.Format == "json" {
		outputFormat = output.FormatJSON
	}
	out := output.NewStdout(outputFormat)

	// Create shared context
	ctx := &types.Context{
		Config: cfg,
		Output: out,
	}

	// Parse command line
	var cli CLI
	kongCtx := kong.Parse(&cli,
		kong.Name("belt"),
		kong.Description("A modern CLI toolkit built with Go."),
		kong.Vars{
			"version": fmt.Sprintf("%s (%s, built %s)", version, commit, date),
		},
		kong.UsageOnError(),
		kong.ConfigureHelp(kong.HelpOptions{
			Compact: true,
		}),
		kong.Bind(ctx),
	)

	// Handle version flag
	if cli.Version {
		fmt.Printf("belt version %s (%s, built %s)\n", version, commit, date)
		return
	}

	// Execute the command
	err = kongCtx.Run()
	if err != nil {
		out.PrintError(fmt.Sprintf("Command failed: %v", err))
		os.Exit(1)
	}
}
