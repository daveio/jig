package binary

import (
	"testing"
)

func TestGetPlatformAlias(t *testing.T) {
	tests := []struct {
		platform string
		want     string
	}{
		{"darwin", "macos"},
		{"linux", "linux"},
		{"windows", "windows"},
	}

	for _, tt := range tests {
		t.Run(tt.platform, func(t *testing.T) {
			if got := getPlatformAlias(tt.platform); got != tt.want {
				t.Errorf("getPlatformAlias() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestGetArchAlias(t *testing.T) {
	tests := []struct {
		arch string
		want string
	}{
		{"x86_64", "amd64"},
		{"aarch64", "arm64"},
		{"arm64", "arm64"},
		{"amd64", "amd64"},
	}

	for _, tt := range tests {
		t.Run(tt.arch, func(t *testing.T) {
			if got := getArchAlias(tt.arch); got != tt.want {
				t.Errorf("getArchAlias() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIsArchive(t *testing.T) {
	tests := []struct {
		name     string
		filename string
		want     bool
	}{
		{"tar.gz file", "hubbit-linux-amd64.tar.gz", true},
		{"tgz file", "hubbit-linux-amd64.tgz", true},
		{"zip file", "hubbit-windows-amd64.zip", true},
		{"tar.bz2 file", "hubbit-linux-amd64.tar.bz2", true},
		{"tar.xz file", "hubbit-linux-amd64.tar.xz", true},
		{"plain binary", "hubbit-linux-amd64", false},
		{"exe file", "hubbit-windows-amd64.exe", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isArchive(tt.filename); got != tt.want {
				t.Errorf("isArchive() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIsBinary(t *testing.T) {
	tests := []struct {
		name     string
		filename string
		want     bool
	}{
		{"plain binary", "hubbit", true},
		{"exe file", "hubbit.exe", true},
		{"archive file", "hubbit.tar.gz", false},
		{"text file", "readme.txt", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isBinary(tt.filename); got != tt.want {
				t.Errorf("isBinary() = %v, want %v", got, tt.want)
			}
		})
	}
}
