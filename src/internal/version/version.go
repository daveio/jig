package version

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"time"
)

// GetVersion attempts to get the version from git tags, falling back to a datetime format
// or "dev-UNKNOWN" if all else fails
func GetVersion() (version string) {
	// Set the ultimate fallback
	version = "dev-UNKNOWN"

	// Use defer to ensure we always have a version, even if a panic occurs
	defer func() {
		if r := recover(); r != nil {
			fmt.Fprintf(os.Stderr, "Error generating version: %v\n", r)
			version = "dev-UNKNOWN"
		}
	}()

	// Try to get version from git tag
	cmd := exec.Command("git", "describe", "--tags", "--abbrev=0")
	var out bytes.Buffer
	cmd.Stdout = &out
	err := cmd.Run()
	if err == nil {
		gitVersion := strings.TrimSpace(out.String())
		if gitVersion != "" {
			return gitVersion
		}
	}

	// Fallback to datetime format
	now := time.Now()
	formattedTime := now.Format("dev-20060102-150405")
	if formattedTime != "" {
		return formattedTime
	}

	// If we get here, the ultimate fallback will be returned
	return
}

// GetCommit attempts to get the current git commit hash
func GetCommit() string {
	cmd := exec.Command("git", "rev-parse", "--short", "HEAD")
	var out bytes.Buffer
	cmd.Stdout = &out
	err := cmd.Run()
	if err == nil {
		commit := strings.TrimSpace(out.String())
		if commit != "" {
			return commit
		}
	}
	return "dev"
}

// GetBuildDate returns the current date in ISO 8601 format
func GetBuildDate() string {
	return time.Now().Format("2006-01-02T15:04:05Z07:00")
}

// Variables to store version information
var (
	Version = GetVersion()
	Commit  = GetCommit()
	Date    = GetBuildDate()
)

// VersionString returns a formatted version string
func VersionString() string {
	return fmt.Sprintf("%s (%s, built %s)", Version, Commit, Date)
}

// CheckVersionFlag checks if the version flag is present in args and prints version info if it is
func CheckVersionFlag(args []string) bool {
	for _, arg := range args {
		if arg == "-V" || arg == "--version" {
			fmt.Printf("belt version %s\n", VersionString())
			return true
		}
	}
	return false
}
