package tls

import (
	"github.com/daveio/belt/src/commands/tls/cert"
	"github.com/daveio/belt/src/commands/tls/ciphers"
)

// Cmd represents the TLS command group.
type Cmd struct {
	Cert    cert.Cmd    `cmd:"" help:"Certificate operations."`
	Ciphers ciphers.Cmd `cmd:"" help:"List supported TLS ciphers (not yet implemented)."`
}
