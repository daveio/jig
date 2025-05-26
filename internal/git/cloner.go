package git

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/daveio/hubbit/pkg/parser"
	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing/transport"
	"github.com/go-git/go-git/v5/plumbing/transport/http"
	"github.com/go-git/go-git/v5/plumbing/transport/ssh"
)

type ClonerOptions struct {
	Protocol       string
	CloneDirectory string
	UseExternalGit bool
	GitHubToken    string
	Verbose        bool
}

type Cloner struct {
	options ClonerOptions
}

func NewCloner(options ClonerOptions) *Cloner {
	return &Cloner{options: options}
}

func (c *Cloner) Clone(repo *parser.RepositoryInfo) error {
	destPath := c.getDestinationPath(repo)

	if _, err := os.Stat(destPath); err == nil {
		return fmt.Errorf("repository already exists at %s", destPath)
	}

	cloneURL := repo.CloneURL(c.options.Protocol)

	if c.options.UseExternalGit {
		return c.cloneWithCLI(cloneURL, destPath)
	}

	return c.cloneWithLibrary(cloneURL, destPath)
}

func (c *Cloner) getDestinationPath(repo *parser.RepositoryInfo) string {
	baseDir := os.ExpandEnv(c.options.CloneDirectory)
	if strings.HasPrefix(baseDir, "~") {
		home, _ := os.UserHomeDir()
		baseDir = strings.Replace(baseDir, "~", home, 1)
	}

	return filepath.Join(baseDir, repo.Host, repo.Owner, repo.Name)
}

func (c *Cloner) cloneWithCLI(cloneURL, destPath string) error {
	if err := os.MkdirAll(filepath.Dir(destPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	args := []string{"clone"}
	if c.options.Verbose {
		args = append(args, "--verbose", "--progress")
	}
	args = append(args, cloneURL, destPath)

	cmd := exec.Command("git", args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("git clone failed: %w", err)
	}

	return nil
}

func (c *Cloner) cloneWithLibrary(cloneURL, destPath string) error {
	if err := os.MkdirAll(filepath.Dir(destPath), 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	cloneOptions := &git.CloneOptions{
		URL:      cloneURL,
		Progress: nil,
	}

	if c.options.Verbose {
		cloneOptions.Progress = os.Stdout
	}

	auth, err := c.getAuth(cloneURL)
	if err != nil {
		return fmt.Errorf("failed to setup authentication: %w", err)
	}
	if auth != nil {
		cloneOptions.Auth = auth
	}

	_, err = git.PlainClone(destPath, false, cloneOptions)
	if err != nil {
		return fmt.Errorf("failed to clone repository: %w", err)
	}

	return nil
}

func (c *Cloner) getAuth(cloneURL string) (transport.AuthMethod, error) {
	if strings.HasPrefix(cloneURL, "git@") || strings.HasPrefix(cloneURL, "ssh://") {
		auth, err := ssh.DefaultAuthBuilder("git")
		if err != nil {
			return nil, fmt.Errorf("failed to setup SSH auth: %w", err)
		}
		return auth, nil
	}

	if c.options.GitHubToken != "" && strings.Contains(cloneURL, "github.com") {
		return &http.BasicAuth{
			Username: "x-access-token",
			Password: c.options.GitHubToken,
		}, nil
	}

	return nil, nil
}
