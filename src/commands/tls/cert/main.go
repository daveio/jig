package cert

import (
	"github.com/daveio/belt/src/commands/tls/cert/req"
	"github.com/daveio/belt/src/commands/tls/cert/selfsign"
)

// Cmd represents the certificate command group.
type Cmd struct {
	Req      req.Cmd      `cmd:"" help:"Generate certificate request (not yet implemented)."`
	Selfsign selfsign.Cmd `cmd:"" help:"Generate self-signed certificate (not yet implemented)."`
}
