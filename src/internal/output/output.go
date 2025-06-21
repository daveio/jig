package output

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/daveio/belt/src/config"
	"github.com/daveio/belt/src/ui"
)

// OutputFormat represents the output format
type OutputFormat string

const (
	FormatAuto   OutputFormat = "auto"
	FormatJSON   OutputFormat = "json"
	FormatPlain  OutputFormat = "plain"
	FormatStyled OutputFormat = "styled"
)

// Writer handles output formatting and writing
type Writer struct {
	format OutputFormat
	writer io.Writer
	config *config.Config
}

// New creates a new output writer
func New(format OutputFormat, writer io.Writer) *Writer {
	cfg := config.Get()
	if format == FormatAuto {
		// Auto-detect format based on pipe flag or config
		if cfg.Output.Format == "json" {
			format = FormatJSON
		} else if !cfg.Output.Color {
			format = FormatPlain
		} else {
			format = FormatStyled
		}
	}

	return &Writer{
		format: format,
		writer: writer,
		config: cfg,
	}
}

// Print outputs a message based on the configured format
func (w *Writer) Print(message string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(map[string]interface{}{
			"message": message,
			"level":   "info",
		})
	case FormatPlain:
		fmt.Fprintln(w.writer, message)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.Base.Render(message))
	}
}

// PrintSuccess outputs a success message
func (w *Writer) PrintSuccess(message string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(map[string]interface{}{
			"message": message,
			"level":   "success",
		})
	case FormatPlain:
		fmt.Fprintln(w.writer, message)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.SuccessStyle.Render("✓ "+message))
	}
}

// PrintError outputs an error message
func (w *Writer) PrintError(message string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(map[string]interface{}{
			"message": message,
			"level":   "error",
		})
	case FormatPlain:
		fmt.Fprintln(w.writer, message)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.ErrorStyle.Render("✗ "+message))
	}
}

// PrintWarning outputs a warning message
func (w *Writer) PrintWarning(message string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(map[string]interface{}{
			"message": message,
			"level":   "warning",
		})
	case FormatPlain:
		fmt.Fprintln(w.writer, message)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.WarningStyle.Render("⚠ "+message))
	}
}

// PrintInfo outputs an info message
func (w *Writer) PrintInfo(message string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(map[string]interface{}{
			"message": message,
			"level":   "info",
		})
	case FormatPlain:
		fmt.Fprintln(w.writer, message)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.InfoStyle.Render("ℹ "+message))
	}
}

// PrintData outputs structured data
func (w *Writer) PrintData(data interface{}) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		w.printJSON(data)
	case FormatPlain:
		fmt.Fprintf(w.writer, "%+v\n", data)
	case FormatStyled:
		// For styled output, format nicely
		switch v := data.(type) {
		case map[string]interface{}:
			w.printMap(v)
		case []interface{}:
			w.printList(v)
		default:
			fmt.Fprintf(w.writer, "%+v\n", data)
		}
	}
}

// PrintHeading outputs a heading
func (w *Writer) PrintHeading(heading string) {
	if w.config.Output.Silent {
		return
	}

	switch w.format {
	case FormatJSON:
		// Skip headings in JSON mode
		return
	case FormatPlain:
		fmt.Fprintln(w.writer, heading)
	case FormatStyled:
		fmt.Fprintln(w.writer, ui.Heading.Render(heading))
	}
}

// printJSON outputs data as JSON
func (w *Writer) printJSON(data interface{}) {
	enc := json.NewEncoder(w.writer)
	enc.SetIndent("", "  ")
	enc.Encode(data)
}

// printMap outputs a map in a styled format
func (w *Writer) printMap(data map[string]interface{}) {
	for key, value := range data {
		keyStyle := ui.Bold.Render(key + ":")
		fmt.Fprintf(w.writer, "%s %v\n", keyStyle, value)
	}
}

// printList outputs a list in a styled format
func (w *Writer) printList(data []interface{}) {
	for i, item := range data {
		bullet := ui.Base.Foreground(ui.Primary).Render("•")
		fmt.Fprintf(w.writer, "%s %v\n", bullet, item)
		_ = i // avoid unused variable warning
	}
}

// NewStdout creates a new output writer for stdout
func NewStdout(format OutputFormat) *Writer {
	return New(format, os.Stdout)
}

// NewStderr creates a new output writer for stderr
func NewStderr(format OutputFormat) *Writer {
	return New(format, os.Stderr)
}
