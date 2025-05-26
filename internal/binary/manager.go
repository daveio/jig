package binary

import (
	"archive/tar"
	"archive/zip"
	"compress/gzip"
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"time"

	"github.com/daveio/hubbit/pkg/parser"
	"github.com/google/go-github/v72/github"
	"gopkg.in/yaml.v3"
)

type ManagerOptions struct {
	GitHubToken string
	Verbose     bool
}

type Manager struct {
	options     ManagerOptions
	client      *github.Client
	storePath   string
	installPath string
}

func NewManager(options ManagerOptions) *Manager {
	var httpClient *http.Client
	if options.GitHubToken != "" {
		httpClient = &http.Client{
			Transport: &roundTripper{
				token: options.GitHubToken,
			},
		}
	}

	home, _ := os.UserHomeDir()

	return &Manager{
		options:     options,
		client:      github.NewClient(httpClient),
		storePath:   filepath.Join(home, ".config", "hubbit", "binaries.yaml"),
		installPath: filepath.Join(home, ".local", "bin"),
	}
}

type roundTripper struct {
	token string
}

func (rt *roundTripper) RoundTrip(req *http.Request) (*http.Response, error) {
	req.Header.Set("Authorization", "Bearer "+rt.token)

	return http.DefaultTransport.RoundTrip(req)
}

func (m *Manager) Get(repo *parser.RepositoryInfo) error {
	ctx := context.Background()

	release, _, err := m.client.Repositories.GetLatestRelease(ctx, repo.Owner, repo.Name)
	if err != nil {
		return fmt.Errorf("failed to get latest release: %w", err)
	}

	asset := m.findBestAsset(release.Assets)
	if asset == nil {
		return fmt.Errorf("no suitable binary found for %s/%s", runtime.GOOS, runtime.GOARCH)
	}

	if m.options.Verbose {
		fmt.Printf("Downloading %s...\n", asset.GetName())
	}

	tmpFile, err := m.downloadAsset(asset)
	if err != nil {
		return fmt.Errorf("failed to download asset: %w", err)
	}
	defer os.Remove(tmpFile)

	binaryPath, err := m.extractAndInstall(tmpFile, asset.GetName(), repo.Name)
	if err != nil {
		return fmt.Errorf("failed to install binary: %w", err)
	}

	binary := &Binary{
		Repository:  fmt.Sprintf("%s/%s", repo.Owner, repo.Name),
		Owner:       repo.Owner,
		Name:        repo.Name,
		Version:     release.GetTagName(),
		InstalledAt: time.Now(),
		UpdatedAt:   time.Now(),
		AssetName:   asset.GetName(),
		InstallPath: binaryPath,
	}

	if err := m.saveBinary(binary); err != nil {
		return fmt.Errorf("failed to save binary info: %w", err)
	}

	return nil
}

func (m *Manager) Update(repo *parser.RepositoryInfo) (bool, error) {
	store, err := m.loadStore()
	if err != nil {
		return false, err
	}

	key := fmt.Sprintf("%s/%s", repo.Owner, repo.Name)
	existing, ok := store.Binaries[key]

	if !ok {
		return false, fmt.Errorf("binary not installed: %s", key)
	}

	ctx := context.Background()

	release, _, err := m.client.Repositories.GetLatestRelease(ctx, repo.Owner, repo.Name)
	if err != nil {
		return false, fmt.Errorf("failed to get latest release: %w", err)
	}

	if existing.Version == release.GetTagName() {
		return false, nil
	}

	if err := m.Get(repo); err != nil {
		return false, err
	}

	return true, nil
}

func (m *Manager) UpdateAll() (int, error) {
	store, err := m.loadStore()
	if err != nil {
		return 0, err
	}

	updated := 0

	for _, binary := range store.Binaries {
		repo := &parser.RepositoryInfo{
			Host:  "github.com",
			Owner: binary.Owner,
			Name:  binary.Name,
		}

		wasUpdated, err := m.Update(repo)
		if err != nil {
			if m.options.Verbose {
				fmt.Printf("Failed to update %s: %v\n", binary.Repository, err)
			}

			continue
		}

		if wasUpdated {
			updated++
		}
	}

	return updated, nil
}

func (m *Manager) findBestAsset(assets []*github.ReleaseAsset) *github.ReleaseAsset {
	osName := runtime.GOOS
	arch := runtime.GOARCH

	if arch == "amd64" {
		arch = "x86_64"
	}

	for _, asset := range assets {
		name := strings.ToLower(asset.GetName())

		if !strings.Contains(name, osName) && !strings.Contains(name, getPlatformAlias(osName)) {
			continue
		}

		if !strings.Contains(name, arch) && !strings.Contains(name, getArchAlias(arch)) {
			continue
		}

		if isArchive(name) || isBinary(name) {
			return asset
		}
	}

	return nil
}

func getPlatformAlias(platform string) string {
	switch platform {
	case "darwin":
		return "macos"
	default:
		return platform
	}
}

func getArchAlias(arch string) string {
	switch arch {
	case "x86_64":
		return "amd64"
	case "aarch64":
		return "arm64"
	default:
		return arch
	}
}

func isArchive(name string) bool {
	exts := []string{".tar.gz", ".tgz", ".zip", ".tar.bz2", ".tar.xz"}
	for _, ext := range exts {
		if strings.HasSuffix(name, ext) {
			return true
		}
	}

	return false
}

func isBinary(name string) bool {
	return !strings.Contains(name, ".") || strings.HasSuffix(name, ".exe")
}

func (m *Manager) downloadAsset(asset *github.ReleaseAsset) (string, error) {
	resp, err := http.Get(asset.GetBrowserDownloadURL())
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	tmpFile, err := os.CreateTemp("", "hubbit-download-*")
	if err != nil {
		return "", err
	}

	_, err = io.Copy(tmpFile, resp.Body)
	tmpFile.Close()

	if err != nil {
		os.Remove(tmpFile.Name())

		return "", err
	}

	return tmpFile.Name(), nil
}

func (m *Manager) extractAndInstall(tmpFile, assetName, binaryName string) (string, error) {
	if err := os.MkdirAll(m.installPath, 0o755); err != nil {
		return "", err
	}

	destPath := filepath.Join(m.installPath, binaryName)

	switch {
	case strings.HasSuffix(assetName, ".tar.gz") || strings.HasSuffix(assetName, ".tgz"):
		return destPath, m.extractTarGz(tmpFile, destPath, binaryName)
	case strings.HasSuffix(assetName, ".zip"):
		return destPath, m.extractZip(tmpFile, destPath, binaryName)
	default:
		return destPath, m.copyBinary(tmpFile, destPath)
	}
}

func (m *Manager) extractTarGz(src, dest, binaryName string) error {
	file, err := os.Open(src)
	if err != nil {
		return err
	}
	defer file.Close()

	gzr, err := gzip.NewReader(file)
	if err != nil {
		return err
	}
	defer gzr.Close()

	tr := tar.NewReader(gzr)

	for {
		header, err := tr.Next()
		if err == io.EOF {
			break
		}

		if err != nil {
			return err
		}

		if header.Typeflag != tar.TypeReg {
			continue
		}

		baseName := filepath.Base(header.Name)
		if baseName == binaryName || strings.HasPrefix(baseName, binaryName) {
			outFile, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o755)
			if err != nil {
				return err
			}

			_, err = io.Copy(outFile, tr)
			outFile.Close()

			return err
		}
	}

	return errors.New("binary not found in archive")
}

func (m *Manager) extractZip(src, dest, binaryName string) error {
	r, err := zip.OpenReader(src)
	if err != nil {
		return err
	}
	defer r.Close()

	for _, f := range r.File {
		baseName := filepath.Base(f.Name)
		if baseName == binaryName || strings.HasPrefix(baseName, binaryName) {
			rc, err := f.Open()
			if err != nil {
				return err
			}

			outFile, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o755)
			if err != nil {
				rc.Close()

				return err
			}

			_, err = io.Copy(outFile, rc)
			rc.Close()
			outFile.Close()

			return err
		}
	}

	return errors.New("binary not found in archive")
}

func (m *Manager) copyBinary(src, dest string) error {
	input, err := os.Open(src)
	if err != nil {
		return err
	}
	defer input.Close()

	output, err := os.OpenFile(dest, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o755)
	if err != nil {
		return err
	}
	defer output.Close()

	_, err = io.Copy(output, input)

	return err
}

func (m *Manager) saveBinary(binary *Binary) error {
	store, _ := m.loadStore()
	if store.Binaries == nil {
		store.Binaries = make(map[string]*Binary)
	}

	key := fmt.Sprintf("%s/%s", binary.Owner, binary.Name)
	store.Binaries[key] = binary

	data, err := yaml.Marshal(store)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(m.storePath), 0o755); err != nil {
		return err
	}

	return os.WriteFile(m.storePath, data, 0o644)
}

func (m *Manager) loadStore() (*BinaryStore, error) {
	store := &BinaryStore{
		Binaries: make(map[string]*Binary),
	}

	data, err := os.ReadFile(m.storePath)
	if err != nil {
		if os.IsNotExist(err) {
			return store, nil
		}

		return nil, err
	}

	if err := yaml.Unmarshal(data, store); err != nil {
		return nil, err
	}

	return store, nil
}
