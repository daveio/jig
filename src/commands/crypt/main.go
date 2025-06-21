package crypt

import (
	"github.com/daveio/belt/src/commands/crypt/random"
	"github.com/daveio/belt/src/commands/crypt/simple"
	"github.com/daveio/belt/src/commands/crypt/wireguard"
)

// Cmd represents the crypt command group.
type Cmd struct {
	Random    random.Cmd    `cmd:"" help:"Random data generation."`
	Simple    simple.Cmd    `cmd:"" help:"Simple encryption/decryption operations."`
	WireGuard wireguard.Cmd `cmd:"" help:"Generate WireGuard keypair."`
}
