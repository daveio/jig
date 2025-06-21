package types

import (
	"github.com/daveio/belt/src/config"
	"github.com/daveio/belt/src/internal/output"
)

// Context provides shared context for all commands.
type Context struct {
	Config *config.Config
	Output *output.Writer
}
