package random

import (
	"github.com/daveio/belt/src/commands/crypt/random/hex"
	"github.com/daveio/belt/src/commands/crypt/random/pw"
)

// Cmd represents the random command group.
type Cmd struct {
	Hex hex.Cmd `cmd:"" help:"Generate random hex string."`
	Pw  pw.Cmd  `cmd:"" help:"Generate random password."`
}
