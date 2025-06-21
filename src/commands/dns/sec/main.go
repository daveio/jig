package sec

import (
	"fmt"

	"github.com/daveio/belt/src/internal/dns"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the DNSSEC validation command.
type Cmd struct {
	Domain string `arg:"" help:"Root domain to check DNSSEC for."`
}

// Run executes the DNSSEC validation command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Use configured DNS server or default
	server := ctx.Config.DNS.Server
	if server == "" {
		server = "1.1.1.1"
	}

	ctx.Output.PrintInfo(fmt.Sprintf("Checking DNSSEC for %s", c.Domain))

	// Validate DNSSEC
	valid, message, err := dns.ValidateDNSSEC(c.Domain, server)

	// Output based on format
	if ctx.Config.Output.Format == "json" {
		data := map[string]interface{}{
			"domain":  c.Domain,
			"valid":   valid,
			"message": message,
		}
		if err != nil {
			data["error"] = err.Error()
		}
		ctx.Output.PrintData(data)
		return nil
	}

	// Pretty output
	if err != nil {
		// Show the message even if there's an error
		if message != "" {
			ctx.Output.PrintError(message)
		}
		return fmt.Errorf("DNSSEC validation error: %w", err)
	}

	if valid {
		ctx.Output.PrintSuccess(message)

		// Show additional info
		ctx.Output.Print("")
		ctx.Output.PrintInfo("DNSSEC provides:")
		ctx.Output.Print("  • Authentication of DNS data")
		ctx.Output.Print("  • Data integrity protection")
		ctx.Output.Print("  • Authenticated denial of existence")
	} else {
		ctx.Output.PrintError(message)

		// Show troubleshooting tips
		ctx.Output.Print("")
		ctx.Output.PrintWarning("Possible reasons:")
		ctx.Output.Print("  • Domain doesn't support DNSSEC")
		ctx.Output.Print("  • DNS server doesn't validate DNSSEC")
		ctx.Output.Print("  • Network filtering DNSSEC responses")
		ctx.Output.Print("")
		ctx.Output.PrintInfo("Try using a DNSSEC-validating resolver like 1.1.1.1 or 8.8.8.8")
	}

	return nil
}
