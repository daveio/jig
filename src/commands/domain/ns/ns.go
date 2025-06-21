package ns

import (
	"fmt"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the domain nameservers command.
type Cmd struct {
	Domain string `arg:"" help:"Domain to check nameservers for."`
}

// Run executes the domain nameservers command.
func (c *Cmd) Run(ctx *types.Context) error {
	return fmt.Errorf("not yet implemented")
}
