package hubbit

import (
	"errors"
	"fmt"
	"os"

	"github.com/daveio/hubbit/internal/git"
	"github.com/daveio/hubbit/pkg/parser"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var cloneCmd = &cobra.Command{
	Use:   "clone <repository>",
	Short: "Clone a Git repository",
	Long: `Clone a Git repository from GitHub or other sources.
	
Repository can be specified as:
  - Simple repository name (e.g., 'repo')
  - Username/repository (e.g., 'username/repo')
  - Full URL (e.g., 'https://github.com/username/repo')
  - SSH URL (e.g., 'git@github.com:username/repo.git')`,
	Args: cobra.ExactArgs(1),
	RunE: runClone,
}

func init() {
	rootCmd.AddCommand(cloneCmd)
}

func runClone(cmd *cobra.Command, args []string) error {
	repoSpec := args[0]

	username := viper.GetString("github.username")
	if username == "" && parser.IsSimpleRepo(repoSpec) {
		return errors.New(
			"GitHub username not configured. Please set it in config or use 'username/repo' format",
		)
	}

	repoInfo, err := parser.ParseRepository(repoSpec, username)
	if err != nil {
		return fmt.Errorf("failed to parse repository: %w", err)
	}

	protocol := viper.GetString("github.protocol")
	cloneDir := os.ExpandEnv(viper.GetString("github.clone_directory"))
	useExternalGit := viper.GetBool("main.external_git")

	cloner := git.NewCloner(git.ClonerOptions{
		Protocol:       protocol,
		CloneDirectory: cloneDir,
		UseExternalGit: useExternalGit,
		GitHubToken:    viper.GetString("github.token"),
		Verbose:        verbose,
	})

	if verbose {
		fmt.Printf("Cloning %s/%s to %s using %s protocol\n",
			repoInfo.Owner, repoInfo.Name, cloneDir, protocol)
	}

	if err := cloner.Clone(repoInfo); err != nil {
		return fmt.Errorf("failed to clone repository: %w", err)
	}

	fmt.Printf("Successfully cloned %s/%s\n", repoInfo.Owner, repoInfo.Name)

	return nil
}
