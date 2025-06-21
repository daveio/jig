package dns

import (
	"github.com/daveio/belt/src/commands/dns/flush"
	"github.com/daveio/belt/src/commands/dns/lookup"
	"github.com/daveio/belt/src/commands/dns/sec"
)

// Cmd represents the DNS command group.
type Cmd struct {
	Flush  flush.Cmd  `cmd:"" help:"Flush DNS cache."`
	Lookup lookup.Cmd `cmd:"" help:"Perform DNS lookup."`
	Sec    sec.Cmd    `cmd:"" help:"Validate DNSSEC for a domain."`
}
