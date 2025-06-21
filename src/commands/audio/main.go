package audio

import (
	"github.com/daveio/belt/src/commands/audio/info"
)

// Cmd represents the audio command group.
type Cmd struct {
	Info info.Cmd `cmd:"" help:"Extract and display audio file metadata."`
}
