package hubbit

import (
	"fmt"

	"github.com/daveio/hubbit/internal/binary"
	"github.com/daveio/hubbit/pkg/parser"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var binaryCmd = &cobra.Command{
	Use:   "binary",
	Short: "Manage GitHub release binaries",
	Long:  `Download, update, and manage GitHub release binaries.`,
}

var binaryGetCmd = &cobra.Command{
	Use:   "get <repository>",
	Short: "Download the latest binary for a repository",
	Args:  cobra.ExactArgs(1),
	RunE:  runBinaryGet,
}

var binaryUpdateCmd = &cobra.Command{
	Use:   "update [repository]",
	Short: "Update binaries",
	Long: `Update binaries. If repository is specified, updates only that binary.
Otherwise, updates all installed binaries.`,
	Args: cobra.MaximumNArgs(1),
	RunE: runBinaryUpdate,
}

func init() {
	rootCmd.AddCommand(binaryCmd)
	binaryCmd.AddCommand(binaryGetCmd)
	binaryCmd.AddCommand(binaryUpdateCmd)
}

func runBinaryGet(cmd *cobra.Command, args []string) error {
	repoSpec := args[0]

	username := viper.GetString("github.username")
	repoInfo, err := parser.ParseRepository(repoSpec, username)
	if err != nil {
		return fmt.Errorf("failed to parse repository: %w", err)
	}

	manager := binary.NewManager(binary.ManagerOptions{
		GitHubToken: viper.GetString("github.token"),
		Verbose:     verbose,
	})

	if verbose {
		fmt.Printf("Downloading latest binary for %s/%s\n", repoInfo.Owner, repoInfo.Name)
	}

	if err := manager.Get(repoInfo); err != nil {
		return fmt.Errorf("failed to download binary: %w", err)
	}

	fmt.Printf("Successfully downloaded binary for %s/%s\n", repoInfo.Owner, repoInfo.Name)
	return nil
}

func runBinaryUpdate(cmd *cobra.Command, args []string) error {
	manager := binary.NewManager(binary.ManagerOptions{
		GitHubToken: viper.GetString("github.token"),
		Verbose:     verbose,
	})

	if len(args) == 0 {
		if verbose {
			fmt.Println("Checking for updates for all installed binaries")
		}

		updated, err := manager.UpdateAll()
		if err != nil {
			return fmt.Errorf("failed to update binaries: %w", err)
		}

		fmt.Printf("Updated %d binaries\n", updated)
		return nil
	}

	repoSpec := args[0]
	username := viper.GetString("github.username")
	repoInfo, err := parser.ParseRepository(repoSpec, username)
	if err != nil {
		return fmt.Errorf("failed to parse repository: %w", err)
	}

	if verbose {
		fmt.Printf("Checking for updates for %s/%s\n", repoInfo.Owner, repoInfo.Name)
	}

	updated, err := manager.Update(repoInfo)
	if err != nil {
		return fmt.Errorf("failed to update binary: %w", err)
	}

	if updated {
		fmt.Printf("Successfully updated binary for %s/%s\n", repoInfo.Owner, repoInfo.Name)
	} else {
		fmt.Printf("Binary for %s/%s is already up to date\n", repoInfo.Owner, repoInfo.Name)
	}

	return nil
}
