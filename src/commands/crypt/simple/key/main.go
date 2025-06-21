package key

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

// Cmd represents the key generation command.
type Cmd struct {
	Write bool `short:"w" help:"Write key to configuration file."`
}

// Run executes the key generation command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Show key backup warning if not warned
	if !ctx.Config.Crypt.Warned {
		ctx.Output.PrintBox("Remember to back up your encryption key!", ui.WarningBox)
	}

	// If just displaying current key
	if !c.Write {
		currentKey := ctx.Config.Crypt.GetKey()
		if currentKey == "" {
			return fmt.Errorf(
				"no encryption key found. Run 'belt init' or use --write to generate one",
			)
		}

		// Output based on format
		if ctx.Config.Output.Format == "json" {
			ctx.Output.PrintData(map[string]interface{}{
				"key": currentKey,
			})
		} else {
			fmt.Println(currentKey)
		}
		return nil
	}

	// Generate new key
	newKey, err := crypto.GenerateKeyBase58()
	if err != nil {
		return fmt.Errorf("generating key: %w", err)
	}

	// Confirm before writing
	var confirm bool
	form := huh.NewForm(
		huh.NewGroup(
			huh.NewConfirm().
				Title("Generate new encryption key?").
				Description("This will replace your existing key. Make sure to back up the old key!").
				Value(&confirm),
		),
	).WithTheme(huh.ThemeBase())

	if err := form.Run(); err != nil {
		return fmt.Errorf("confirmation prompt: %w", err)
	}

	if !confirm {
		ctx.Output.PrintInfo("Key generation cancelled.")
		return nil
	}

	// Update configuration
	configPath, err := config.GetConfigPath()
	if err != nil {
		return fmt.Errorf("getting config path: %w", err)
	}

	// Read existing config
	data, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("reading config: %w", err)
	}

	// Parse YAML
	var cfg map[string]interface{}
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		return fmt.Errorf("parsing config: %w", err)
	}

	// Update key
	if cryptCfg, ok := cfg["crypt"].(map[string]interface{}); ok {
		cryptCfg["key"] = newKey
		cryptCfg["warned"] = false // Reset warning
	} else {
		cfg["crypt"] = map[string]interface{}{
			"env":    "BELT_CRYPT_KEY",
			"key":    newKey,
			"warned": false,
		}
	}

	// Create directory if needed
	configDir := filepath.Dir(configPath)
	if err := os.MkdirAll(configDir, 0o755); err != nil {
		return fmt.Errorf("creating config directory: %w", err)
	}

	// Write updated config
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

	// Show success with warning
	ctx.Output.PrintSuccess("New encryption key generated!")
	warningMsg := fmt.Sprintf(
		"IMPORTANT: Back up your new key!\n\nKey: %s\n\n"+
			"Store this key in a secure location.\n"+
			"You can also set it as an environment variable:\n"+
			"export BELT_CRYPT_KEY=%s",
		newKey, newKey)
	ctx.Output.PrintBox(warningMsg, ui.WarningBox)

	return nil
}
