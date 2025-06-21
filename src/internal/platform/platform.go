package platform

import (
	"fmt"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

// OS represents the operating system type.
type OS string

const (
	OSWindows OS = "windows"
	OSMacOS   OS = "darwin"
	OSLinux   OS = "linux"
	OSUnknown OS = "unknown"
)

// GetOS returns the current operating system.
func GetOS() OS {
	switch runtime.GOOS {
	case "windows":
		return OSWindows
	case "darwin":
		return OSMacOS
	case "linux":
		return OSLinux
	default:
		return OSUnknown
	}
}

// IsWindows returns true if running on Windows.
func IsWindows() bool {
	return GetOS() == OSWindows
}

// IsMacOS returns true if running on macOS.
func IsMacOS() bool {
	return GetOS() == OSMacOS
}

// IsLinux returns true if running on Linux.
func IsLinux() bool {
	return GetOS() == OSLinux
}

// IsUnix returns true if running on a Unix-like system (macOS or Linux).
func IsUnix() bool {
	return IsMacOS() || IsLinux()
}

// RunCommand executes a command and returns output and error.
func RunCommand(name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return string(output), fmt.Errorf("command failed: %w\nOutput: %s", err, output)
	}
	return strings.TrimSpace(string(output)), nil
}

// RunCommandWithSudo runs a command with sudo on Unix systems.
func RunCommandWithSudo(name string, args ...string) (string, error) {
	if IsWindows() {
		// On Windows, just run the command directly
		return RunCommand(name, args...)
	}

	// Check if we're already root
	if os.Geteuid() == 0 {
		return RunCommand(name, args...)
	}

	// Prepend sudo
	sudoArgs := append([]string{name}, args...)
	return RunCommand("sudo", sudoArgs...)
}

// CommandExists checks if a command exists in PATH.
func CommandExists(name string) bool {
	_, err := exec.LookPath(name)
	return err == nil
}

// GetDNSFlushCommand returns the platform-specific DNS flush command.
func GetDNSFlushCommand() ([]string, bool) {
	switch GetOS() {
	case OSWindows:
		return []string{"ipconfig", "/flushdns"}, false
	case OSMacOS:
		// macOS needs multiple commands
		return []string{"dscacheutil", "-flushcache"}, true
	case OSLinux:
		// Try systemd-resolve first
		if CommandExists("systemd-resolve") {
			return []string{"systemd-resolve", "--flush-caches"}, true
		}
		// Try nscd
		if CommandExists("nscd") {
			return []string{"nscd", "-i", "hosts"}, true
		}
		// Try service command
		if CommandExists("service") {
			return []string{"service", "nscd", "restart"}, true
		}
		return nil, false
	default:
		return nil, false
	}
}

// GetMacOSDNSCommands returns all DNS flush commands for macOS.
func GetMacOSDNSCommands() [][]string {
	return [][]string{
		{"dscacheutil", "-flushcache"},
		{"killall", "-HUP", "mDNSResponder"},
	}
}
