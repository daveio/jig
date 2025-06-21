package format

import (
	"github.com/daveio/belt/src/commands/format/json"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the format command.
type Cmd struct {
	JSON json.Cmd `cmd:"" help:"Format JSON data."`
}

// Run executes the format command (this is a container command).
func (c *Cmd) Run(ctx *types.Context) error {
	// This should not be called directly as format has subcommands
	ctx.Output.PrintInfo("Use 'belt format --help' to see available subcommands")

	return nil
}
