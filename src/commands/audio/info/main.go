package info

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/daveio/belt/src/internal/audio"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the audio info command.
type Cmd struct {
	Path string `arg:"" help:"File or directory path to analyze." type:"path"`
}

// Run executes the audio info command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Check if path exists
	info, err := os.Stat(c.Path)
	if err != nil {
		return fmt.Errorf("accessing path: %w", err)
	}

	// If it's a file, process it directly
	if !info.IsDir() {
		if !audio.IsAudioFile(c.Path) {
			return fmt.Errorf("not an audio file: %s", c.Path)
		}
		return c.processFile(ctx, c.Path)
	}

	// If it's a directory, walk the tree
	return audio.WalkAudioFiles(c.Path, func(path string) error {
		return c.processFile(ctx, path)
	})
}

// processFile extracts and outputs audio file information.
func (c *Cmd) processFile(ctx *types.Context, path string) error {
	audioInfo, err := audio.GetAudioInfo(path)
	if err != nil {
		// Don't fail the whole operation for one bad file
		if ctx.Output != nil && !ctx.Config.Output.Silent {
			ctx.Output.PrintError(fmt.Sprintf("Error processing %s: %v", path, err))
		}
		return nil
	}

	// Convert to absolute path for consistent output
	absPath, err := filepath.Abs(path)
	if err != nil {
		absPath = path
	}
	audioInfo.Path = absPath

	// Format and output
	output := audio.FormatAudioInfo(audioInfo)

	// For pipe mode, output raw data
	if ctx.Config.Output.Format == "json" {
		ctx.Output.PrintData(map[string]interface{}{
			"path":            audioInfo.Path,
			"sample_rate":     audioInfo.SampleRate,
			"bits_per_sample": audioInfo.BitsPerSample,
			"file_size":       audioInfo.FileSize,
		})
	} else {
		// Standard output format
		fmt.Println(output)
	}

	return nil
}
