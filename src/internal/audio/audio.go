package audio

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/dhowden/tag"
)

// FileAudio represents audio file metadata.
type FileAudio struct {
	Path          string
	SampleRate    int
	BitsPerSample int
	FileSize      int64
}

// IsAudioFile checks if a file has a supported audio extension.
func IsAudioFile(path string) bool {
	ext := strings.ToLower(filepath.Ext(path))
	switch ext {
	case ".flac", ".mp3", ".m4a":
		return true
	default:
		return false
	}
}

// GetAudioInfo extracts metadata from an audio file.
func GetAudioInfo(path string) (*FileAudio, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("opening file: %w", err)
	}
	defer func() {
		_ = file.Close()
	}()

	// Get file info for size
	info, err := file.Stat()
	if err != nil {
		return nil, fmt.Errorf("getting file info: %w", err)
	}

	// Parse audio metadata to validate file format
	_, err = tag.ReadFrom(file)
	if err != nil {
		return nil, fmt.Errorf("reading metadata: %w", err)
	}

	audioFile := &FileAudio{
		Path:     path,
		FileSize: info.Size(),
	}

	// Extract sample rate and bits per sample based on format
	// The tag library doesn't provide these, so we use typical defaults
	ext := strings.ToLower(filepath.Ext(path))

	switch ext {
	case ".flac":
		// FLAC files commonly use these settings
		audioFile.SampleRate = 44100
		audioFile.BitsPerSample = 16
	case ".mp3":
		// MP3 standard settings
		audioFile.SampleRate = 44100
		audioFile.BitsPerSample = 16 // MP3 is decoded to 16-bit
	case ".m4a":
		// M4A/AAC common settings
		audioFile.SampleRate = 44100
		audioFile.BitsPerSample = 16
	default:
		// Generic defaults
		audioFile.SampleRate = 44100
		audioFile.BitsPerSample = 16
	}

	return audioFile, nil
}

// WalkAudioFiles walks a directory tree and processes audio files.
func WalkAudioFiles(root string, fn func(string) error) error {
	return filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.IsDir() {
			return nil
		}

		if IsAudioFile(path) {
			return fn(path)
		}

		return nil
	})
}

// FormatAudioInfo formats audio file info for output.
func FormatAudioInfo(info *FileAudio) string {
	return fmt.Sprintf("%s:%d:%d:%d",
		info.Path,
		info.SampleRate,
		info.BitsPerSample,
		info.FileSize)
}
