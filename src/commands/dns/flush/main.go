package flush

import (
	"fmt"

	"github.com/daveio/belt/src/internal/platform"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the DNS flush command.
type Cmd struct{}

// Run executes the DNS flush command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Get platform-specific DNS flush command
	commands, needsSudo := platform.GetDNSFlushCommand()
	if commands == nil {
		return fmt.Errorf("DNS flush not supported on %s", platform.GetOS())
	}

	// Show what we're doing
	ctx.Output.PrintInfo("Flushing DNS cache...")

	// Special handling for macOS which needs multiple commands
	if platform.IsMacOS() {
		for _, cmd := range platform.GetMacOSDNSCommands() {
			// Show command being executed
			ctx.Output.PrintCommand(fmt.Sprintf("sudo %s", joinCmd(cmd)))

			// Execute with sudo
			output, err := platform.RunCommandWithSudo(cmd[0], cmd[1:]...)
			if err != nil {
				return fmt.Errorf("running %s: %w", cmd[0], err)
			}

			if output != "" {
				ctx.Output.Print(output)
			}
		}
		ctx.Output.PrintSuccess("DNS cache flushed successfully!")
		return nil
	}

	// For other platforms
	cmdStr := joinCmd(commands)
	if needsSudo && platform.IsUnix() {
		ctx.Output.PrintCommand("sudo " + cmdStr)
		output, err := platform.RunCommandWithSudo(commands[0], commands[1:]...)
		if err != nil {
			return fmt.Errorf("flushing DNS cache: %w", err)
		}
		if output != "" {
			ctx.Output.Print(output)
		}
	} else {
		ctx.Output.PrintCommand(cmdStr)
		output, err := platform.RunCommand(commands[0], commands[1:]...)
		if err != nil {
			return fmt.Errorf("flushing DNS cache: %w", err)
		}
		if output != "" {
			ctx.Output.Print(output)
		}
	}

	ctx.Output.PrintSuccess("DNS cache flushed successfully!")

	// Show platform-specific notes
	switch platform.GetOS() {
	case platform.OSWindows:
		ctx.Output.PrintInfo("Note: Some applications may cache DNS separately")
	case platform.OSLinux:
		if platform.CommandExists("systemd-resolve") {
			ctx.Output.PrintInfo("Note: systemd-resolved cache cleared")
		}
	}

	return nil
}

// joinCmd joins command parts into a string for display.
func joinCmd(parts []string) string {
	result := ""
	for i, part := range parts {
		if i > 0 {
			result += " "
		}
		result += part
	}
	return result
}
