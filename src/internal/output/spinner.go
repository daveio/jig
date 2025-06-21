package output

import (
	"fmt"
	"io"
	"strings"
	"time"

	"github.com/daveio/belt/src/ui"
)

// Spinner provides a loading animation.
type Spinner struct {
	frames  []string
	current int
	message string
	writer  io.Writer
	stop    chan bool
	done    chan bool
}

// NewSpinner creates a new spinner.
func NewSpinner(message string, writer io.Writer) *Spinner {
	return &Spinner{
		frames:  ui.SpinnerFrames,
		message: message,
		writer:  writer,
		stop:    make(chan bool),
		done:    make(chan bool),
	}
}

// Start begins the spinner animation.
func (s *Spinner) Start() {
	go func() {
		ticker := time.NewTicker(100 * time.Millisecond)
		defer ticker.Stop()

		for {
			select {
			case <-s.stop:
				// Clear the line
				clearLength := len(s.frames[s.current]) + len(s.message) + 2
				_, _ = fmt.Fprintf(s.writer, "\r%s\r", strings.Repeat(" ", clearLength))
				s.done <- true
				return
			case <-ticker.C:
				frame := ui.Base.Foreground(ui.Primary).Render(s.frames[s.current])
				_, _ = fmt.Fprintf(s.writer, "\r%s %s", frame, s.message)
				s.current = (s.current + 1) % len(s.frames)
			}
		}
	}()
}

// Stop halts the spinner animation.
func (s *Spinner) Stop() {
	s.stop <- true
	<-s.done
}

// Success stops the spinner and shows a success message.
func (s *Spinner) Success(message string) {
	s.Stop()
	check := ui.SuccessStyle.Render("✓")
	_, _ = fmt.Fprintf(s.writer, "%s %s\n", check, message)
}

// Error stops the spinner and shows an error message.
func (s *Spinner) Error(message string) {
	s.Stop()
	cross := ui.ErrorStyle.Render("✗")
	_, _ = fmt.Fprintf(s.writer, "%s %s\n", cross, message)
}
