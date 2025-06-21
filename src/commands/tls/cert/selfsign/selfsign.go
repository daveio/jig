package selfsign

import (
	"fmt"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the self-sign certificate command.
type Cmd struct{}

// Run executes the self-sign certificate command.
func (c *Cmd) Run(ctx *types.Context) error {
	return fmt.Errorf("not yet implemented")
}
