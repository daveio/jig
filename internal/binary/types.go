package binary

import "time"

type Binary struct {
	Repository  string    `yaml:"repository"`
	Owner       string    `yaml:"owner"`
	Name        string    `yaml:"name"`
	Version     string    `yaml:"version"`
	InstalledAt time.Time `yaml:"installed_at"`
	UpdatedAt   time.Time `yaml:"updated_at"`
	AssetName   string    `yaml:"asset_name"`
	InstallPath string    `yaml:"install_path"`
}

type BinaryStore struct {
	Binaries map[string]*Binary `yaml:"binaries"`
}
