package simple

import (
	"github.com/daveio/belt/src/commands/crypt/simple/decrypt"
	"github.com/daveio/belt/src/commands/crypt/simple/encrypt"
	"github.com/daveio/belt/src/commands/crypt/simple/key"
)

// Cmd represents the simple encryption command group.
type Cmd struct {
	Encrypt encrypt.Cmd `cmd:"" help:"Encrypt data from stdin."`
	Decrypt decrypt.Cmd `cmd:"" help:"Decrypt data from stdin."`
	Key     key.Cmd     `cmd:"" help:"Generate new encryption key."`
}
