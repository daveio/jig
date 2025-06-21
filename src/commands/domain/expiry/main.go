package expiry

import (
	"fmt"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the domain expiry command.
type Cmd struct {
	Domain string `arg:"" help:"Domain to check expiry for."`
}

// Run executes the domain expiry command.
func (c *Cmd) Run(ctx *types.Context) error {
	return fmt.Errorf("not yet implemented")
}
