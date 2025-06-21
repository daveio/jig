package req

import (
	"fmt"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the certificate request command.
type Cmd struct{}

// Run executes the certificate request command.
func (c *Cmd) Run(ctx *types.Context) error {
	return fmt.Errorf("not yet implemented")
}
