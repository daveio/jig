package hello

import (
	"fmt"
	"strings"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the hello command
type Cmd struct {
	Name string `arg:"" optional:"" help:"Name to greet (default: World)" default:"World"`
	Loud bool   `short:"L" help:"Make the greeting loud (uppercase)"`
}

// Run executes the hello command
func (c *Cmd) Run(ctx *types.Context) error {
	greeting := fmt.Sprintf("Hello, %s!", c.Name)

	if c.Loud {
		greeting = strings.ToUpper(greeting)
	}

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo(fmt.Sprintf("Greeting person: %s", c.Name))
	}

	ctx.Output.PrintSuccess(greeting)

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo("Hello command completed successfully")
	}

	return nil
}
