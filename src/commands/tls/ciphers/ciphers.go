package ciphers

import (
	"fmt"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the TLS ciphers command.
type Cmd struct{}

// Run executes the TLS ciphers command.
func (c *Cmd) Run(ctx *types.Context) error {
	return fmt.Errorf("not yet implemented")
}
