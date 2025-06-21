package json

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the JSON format command.
type Cmd struct {
	File     string `arg:"" help:"JSON file to format (default: stdin)" optional:"" type:"path"`
	Indent   string `       help:"Indentation string"                                           default:"  "`
	Compact  bool   `       help:"Compact output (no indentation)"                                           short:"c"`
	Validate bool   `       help:"Only validate JSON, don't format"                                          short:"V"`
}

// Run executes the JSON format command.
func (c *Cmd) Run(ctx *types.Context) error {
	if ctx.Config.App.Debug {
		if c.File != "" {
			ctx.Output.PrintInfo("Formatting JSON file: " + c.File)
		} else {
			ctx.Output.PrintInfo("Formatting JSON from stdin")
		}
	}

	// Read input
	var input []byte

	var err error

	if c.File != "" {
		input, err = os.ReadFile(c.File)
		if err != nil {
			return fmt.Errorf("failed to read file %s: %w", c.File, err)
		}
	} else {
		input, err = io.ReadAll(os.Stdin)
		if err != nil {
			return fmt.Errorf("failed to read from stdin: %w", err)
		}
	}

	// Parse JSON
	var data interface{}
	if err := json.Unmarshal(input, &data); err != nil {
		return fmt.Errorf("invalid JSON: %w", err)
	}

	if c.Validate {
		ctx.Output.PrintSuccess("JSON is valid")

		return nil
	}

	// Format JSON
	var formatted []byte
	if c.Compact {
		formatted, err = json.Marshal(data)
	} else {
		formatted, err = json.MarshalIndent(data, "", c.Indent)
	}

	if err != nil {
		return fmt.Errorf("failed to format JSON: %w", err)
	}

	// Output formatted JSON
	output := strings.TrimSpace(string(formatted))
	ctx.Output.Print(output)

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo("JSON formatting completed successfully")
	}

	return nil
}
