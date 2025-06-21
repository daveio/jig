package init

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/charmbracelet/huh"
	"github.com/daveio/belt/src/config"
	"github.com/daveio/belt/src/internal/crypto"
	"github.com/daveio/belt/src/internal/types"
	"github.com/daveio/belt/src/ui"
	"gopkg.in/yaml.v3"
)

// Cmd represents the init command.
type Cmd struct {
	Write bool `short:"w" help:"Overwrite existing config without confirmation."`
}

// Run executes the init command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Check if config already exists
	configPath, err := config.GetConfigPath()
	if err != nil {
		return fmt.Errorf("getting config path: %w", err)
	}

	configExists := config.ConfigExists()

	// If config exists and no --write flag, prompt for confirmation
	if configExists && !c.Write {
		ctx.Output.PrintWarning("Configuration file already exists at: " + configPath)

		var confirm bool
		form := huh.NewForm(
			huh.NewGroup(
				huh.NewConfirm().
					Title("Overwrite existing configuration?").
					Value(&confirm),
			),
		).WithTheme(huh.ThemeBase())

		if err := form.Run(); err != nil {
			return fmt.Errorf("confirmation prompt: %w", err)
		}

		if !confirm {
			ctx.Output.PrintInfo("Configuration initialization cancelled.")
			return nil
		}
	}

	// Generate new encryption key
	key, err := crypto.GenerateKeyBase58()
	if err != nil {
		return fmt.Errorf("generating encryption key: %w", err)
	}

	// Create config structure
	cfg := map[string]interface{}{
		"crypt": map[string]interface{}{
			"env":    "BELT_CRYPT_KEY",
			"key":    key,
			"warned": false,
		},
		"dns": map[string]interface{}{
			"server": "1.1.1.1",
			"root":   false,
		},
	}

	// Create directory if needed
	configDir := filepath.Dir(configPath)
	if err := os.MkdirAll(configDir, 0o755); err != nil {
		return fmt.Errorf("creating config directory: %w", err)
	}

	// Write YAML configuration
	file, err := os.Create(configPath)
	if err != nil {
		return fmt.Errorf("creating config file: %w", err)
	}
	defer func() {
		if err := file.Close(); err != nil {
			ctx.Output.PrintError(fmt.Sprintf("Error closing config file: %v", err))
		}
	}()

	encoder := yaml.NewEncoder(file)
	encoder.SetIndent(2)
	if err := encoder.Encode(cfg); err != nil {
		return fmt.Errorf("writing config: %w", err)
	}

	// Show success message with key backup warning
	ctx.Output.PrintSuccess("Configuration initialized successfully!")
	ctx.Output.PrintInfo("Config file: " + configPath)

	// Show key backup warning in a box
	warningMsg := fmt.Sprintf(
		"IMPORTANT: Back up your encryption key!\n\nKey: %s\n\n"+
			"Store this key in a secure location.\n"+
			"You can also set it as an environment variable:\n"+
			"export BELT_CRYPT_KEY=%s",
		key, key)
	ctx.Output.PrintBox(warningMsg, ui.WarningBox)

	return nil
}
