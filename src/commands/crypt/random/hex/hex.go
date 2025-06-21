package hex

import (
	"fmt"

	"github.com/daveio/belt/src/internal/crypto"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the random hex command.
type Cmd struct {
	Length int `arg:"" optional:"" default:"16" help:"Number of bytes to generate (default: 16)."`
}

// Run executes the random hex command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Validate length
	if c.Length <= 0 {
		return fmt.Errorf("length must be positive")
	}

	// Generate random hex
	hexStr, err := crypto.GenerateRandomHex(c.Length)
	if err != nil {
		return fmt.Errorf("generating random hex: %w", err)
	}

	// Output based on format
	if ctx.Config.Output.Format == "json" {
		ctx.Output.PrintData(map[string]interface{}{
			"hex":    hexStr,
			"bytes":  c.Length,
			"length": len(hexStr),
		})
	} else {
		// Direct output to stdout for scripting
		fmt.Println(hexStr)
	}

	return nil
}
