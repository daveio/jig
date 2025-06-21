package list

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the list command
type Cmd struct {
	Path    string `arg:"" optional:"" help:"Path to list (default: current directory)" default:"." type:"path"`
	Hidden  bool   `short:"H" help:"Show hidden files"`
	Long    bool   `short:"L" help:"Show detailed information"`
	Recurse bool   `short:"R" help:"List recursively"`
}

// FileInfo represents file information for display
type FileInfo struct {
	Name    string `json:"name"`
	Path    string `json:"path"`
	Size    int64  `json:"size"`
	IsDir   bool   `json:"is_dir"`
	Mode    string `json:"mode,omitempty"`
	ModTime string `json:"mod_time,omitempty"`
}

// Run executes the list command
func (c *Cmd) Run(ctx *types.Context) error {
	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo(fmt.Sprintf("Listing path: %s", c.Path))
	}

	files, err := c.listFiles(c.Path)
	if err != nil {
		return fmt.Errorf("failed to list files: %w", err)
	}

	if len(files) == 0 {
		ctx.Output.PrintInfo("No files found")
		return nil
	}

	// Sort files by name
	sort.Slice(files, func(i, j int) bool {
		return files[i].Name < files[j].Name
	})

	// Output based on format
	if ctx.Config.Output.Format == "json" {
		ctx.Output.PrintData(files)
	} else {
		c.printFiles(ctx, files)
	}

	if ctx.Config.App.Debug {
		ctx.Output.PrintInfo(fmt.Sprintf("Listed %d files", len(files)))
	}

	return nil
}

// listFiles lists files in the specified path
func (c *Cmd) listFiles(path string) ([]FileInfo, error) {
	var files []FileInfo

	if c.Recurse {
		err := filepath.Walk(path, func(walkPath string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}

			if !c.Hidden && info.Name()[0] == '.' && walkPath != path {
				if info.IsDir() {
					return filepath.SkipDir
				}
				return nil
			}

			files = append(files, c.fileInfoFromOS(walkPath, info))
			return nil
		})
		return files, err
	}

	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}

	for _, entry := range entries {
		if !c.Hidden && entry.Name()[0] == '.' {
			continue
		}

		info, err := entry.Info()
		if err != nil {
			continue
		}

		fullPath := filepath.Join(path, entry.Name())
		files = append(files, c.fileInfoFromOS(fullPath, info))
	}

	return files, nil
}

// fileInfoFromOS converts os.FileInfo to our FileInfo struct
func (c *Cmd) fileInfoFromOS(path string, info os.FileInfo) FileInfo {
	fi := FileInfo{
		Name:  info.Name(),
		Path:  path,
		Size:  info.Size(),
		IsDir: info.IsDir(),
	}

	if c.Long {
		fi.Mode = info.Mode().String()
		fi.ModTime = info.ModTime().Format("2006-01-02 15:04:05")
	}

	return fi
}

// printFiles prints files in a styled format
func (c *Cmd) printFiles(ctx *types.Context, files []FileInfo) {
	for _, file := range files {
		if c.Long {
			ctx.Output.Print(fmt.Sprintf("%s %8d %s %s",
				file.Mode,
				file.Size,
				file.ModTime,
				file.Name,
			))
		} else {
			if file.IsDir {
				ctx.Output.PrintInfo(fmt.Sprintf("📁 %s/", file.Name))
			} else {
				ctx.Output.Print(fmt.Sprintf("📄 %s", file.Name))
			}
		}
	}
}
