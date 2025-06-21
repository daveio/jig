package hello

import (
	"fmt"
	"strings"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the hello command.
type Cmd struct {
	Name string `arg:"" default:"World" help:"Name to greet (default: World)"     optional:""`
	Loud bool   `                       help:"Make the greeting loud (uppercase)"             short:"L"`
}

// Run executes the hello command.
func (c *Cmd) Run(ctx *types.Context) error {
	greeting := fmt.Sprintf("Hello, %s!", c.Name)

	if c.Loud {
		greeting = strings.ToUpper(greeting)
	}

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo("Greeting person: " + c.Name)
	}

	ctx.Output.PrintSuccess(greeting)

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo("Hello command completed successfully")
	}

	return nil
}
