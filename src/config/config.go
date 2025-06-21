package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/adrg/xdg"
	"github.com/knadh/koanf/parsers/json"
	"github.com/knadh/koanf/parsers/yaml"
	"github.com/knadh/koanf/providers/env"
	"github.com/knadh/koanf/providers/file"
	"github.com/knadh/koanf/providers/structs"
	"github.com/knadh/koanf/v2"
)

// Config holds the application configuration.
type Config struct {
	App     AppConfig     `koanf:"app"`
	Output  OutputConfig  `koanf:"output"`
	Logging LoggingConfig `koanf:"logging"`
	Crypt   CryptConfig   `koanf:"crypt"`
	DNS     DNSConfig     `koanf:"dns"`
}

// AppConfig holds application-specific settings.
type AppConfig struct {
	Name    string `koanf:"name"`
	Version string `koanf:"version"`
	Debug   bool   `koanf:"debug"`
}

// OutputConfig holds output formatting settings.
type OutputConfig struct {
	Format string `koanf:"format"`
	Color  bool   `koanf:"color"`
	Quiet  bool   `koanf:"quiet"`
	Silent bool   `koanf:"silent"`
}

// LoggingConfig holds logging settings.
type LoggingConfig struct {
	Level  string `koanf:"level"`
	Format string `koanf:"format"`
}

// CryptConfig holds cryptography settings.
type CryptConfig struct {
	Env    string `koanf:"env"`    // Environment variable name for encryption key
	Key    string `koanf:"key"`    // Base58-encoded encryption key
	Warned bool   `koanf:"warned"` // Whether user has been warned about key backup
}

// DNSConfig holds DNS settings.
type DNSConfig struct {
	Server string `koanf:"server"` // Default DNS server
	Root   bool   `koanf:"root"`   // Whether to use root servers by default
}

var (
	k      *koanf.Koanf
	config *Config
)

// Load loads configuration from multiple sources in order of precedence:
// 1. Environment variables (BELT_*)
// 2. Config files (belt.yaml, belt.json)
// 3. Default values.
func Load() (*Config, error) {
	k = koanf.New(".")

	// Load default values first
	defaults := getDefaults()
	if err := k.Load(structs.Provider(defaults, "koanf"), nil); err != nil {
		return nil, fmt.Errorf("loading defaults: %w", err)
	}

	// Load config files (belt.yaml, belt.json)
	if err := loadConfigFiles(); err != nil {
		return nil, fmt.Errorf("loading config files: %w", err)
	}

	// Load environment variables with BELT_ prefix
	if err := k.Load(env.Provider("BELT_", ".", func(s string) string {
		return strings.ToLower(strings.TrimPrefix(s, "BELT_"))
	}), nil); err != nil {
		return nil, fmt.Errorf("loading environment variables: %w", err)
	}

	// Unmarshal to struct
	config = &Config{}
	if err := k.Unmarshal("", config); err != nil {
		return nil, fmt.Errorf("unmarshaling config: %w", err)
	}

	return config, nil
}

// Get returns the loaded configuration.
func Get() *Config {
	if config == nil {
		panic("configuration not loaded - call Load() first")
	}

	return config
}

// getDefaults returns the default configuration values.
func getDefaults() Config {
	return Config{
		App: AppConfig{
			Name:    "belt",
			Version: "1.0.0",
			Debug:   false,
		},
		Output: OutputConfig{
			Format: "auto",
			Color:  true,
			Quiet:  false,
			Silent: false,
		},
		Logging: LoggingConfig{
			Level:  "info",
			Format: "text",
		},
		Crypt: CryptConfig{
			Env:    "BELT_CRYPT_KEY",
			Key:    "",
			Warned: false,
		},
		DNS: DNSConfig{
			Server: "1.1.1.1",
			Root:   false,
		},
	}
}

// loadConfigFiles loads configuration from standard locations.
func loadConfigFiles() error {
	// Get XDG config paths
	xdgYaml, _ := xdg.ConfigFile("belt/config.yaml")
	xdgYml, _ := xdg.ConfigFile("belt/config.yml")
	xdgJson, _ := xdg.ConfigFile("belt/config.json")

	configPaths := []string{
		"belt.yaml",
		"belt.yml",
		"belt.json",
		xdgYaml,
		xdgYml,
		xdgJson,
	}

	for _, path := range configPaths {
		if _, err := os.Stat(path); os.IsNotExist(err) {
			continue
		}

		var parser koanf.Parser

		switch filepath.Ext(path) {
		case ".yaml", ".yml":
			parser = yaml.Parser()
		case ".json":
			parser = json.Parser()
		default:
			continue
		}

		if err := k.Load(file.Provider(path), parser); err != nil {
			return fmt.Errorf("loading config file %s: %w", path, err)
		}

		break // Use first found config file
	}

	return nil
}

// GetKey returns the encryption key, checking environment variable first.
func (c *CryptConfig) GetKey() string {
	if c.Env != "" {
		if envKey := os.Getenv(c.Env); envKey != "" {
			return envKey
		}
	}
	return c.Key
}

// GetConfigPath returns the path to the belt configuration file.
func GetConfigPath() (string, error) {
	return xdg.ConfigFile("belt/config.yaml")
}

// ConfigExists returns true if the config file exists.
func ConfigExists() bool {
	path, err := GetConfigPath()
	if err != nil {
		return false
	}
	_, err = os.Stat(path)
	return err == nil
}
