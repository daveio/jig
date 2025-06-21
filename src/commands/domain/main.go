package domain

import (
	"github.com/daveio/belt/src/commands/domain/expiry"
	"github.com/daveio/belt/src/commands/domain/ns"
)

// Cmd represents the domain command group.
type Cmd struct {
	Expiry expiry.Cmd `cmd:"" help:"Check domain expiry date (not yet implemented)."`
	NS     ns.Cmd     `cmd:"" help:"Check domain nameservers (not yet implemented)."`
}
