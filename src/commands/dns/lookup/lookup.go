package lookup

import (
	"fmt"
	"strings"

	"github.com/daveio/belt/src/internal/dns"
	"github.com/daveio/belt/src/internal/types"
	mdns "github.com/miekg/dns"
)

// Cmd represents the DNS lookup command.
type Cmd struct {
	Query      string `arg:"" help:"Domain name or IP to query."`
	RecordType string `arg:"" help:"DNS record type (default: A)."                        optional:"" default:"A"`
	Server     string `       help:"DNS server to use (default: from config or 1.1.1.1)."                         short:"e"`
	Root       bool   `       help:"Use root servers directly."                                                   short:"t"`
}

// Run executes the DNS lookup command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Determine server to use
	server := c.Server
	if server == "" {
		if c.Root || ctx.Config.DNS.Root {
			// Use a root server
			server = "198.41.0.4" // a.root-servers.net
		} else if ctx.Config.DNS.Server != "" {
			server = ctx.Config.DNS.Server
		} else {
			server = "1.1.1.1"
		}
	}

	// Create resolver
	resolver := dns.NewResolver(server)

	// Parse record type
	recordType := dns.ParseRecordType(c.RecordType)

	// Perform query
	ctx.Output.PrintInfo(
		fmt.Sprintf("Querying %s for %s records of %s", server, c.RecordType, c.Query),
	)

	resp, err := resolver.Query(c.Query, recordType)
	if err != nil {
		return fmt.Errorf("DNS query failed: %w", err)
	}

	// Check response code
	if resp.Rcode != mdns.RcodeSuccess {
		return fmt.Errorf("DNS query returned: %s", mdns.RcodeToString[resp.Rcode])
	}

	// Output results
	if len(resp.Answer) == 0 {
		ctx.Output.PrintWarning("No records found")
		return nil
	}

	// Format output
	if ctx.Config.Output.Format == "json" {
		var records []map[string]interface{}
		for _, rr := range resp.Answer {
			records = append(records, parseRecord(rr))
		}
		ctx.Output.PrintData(map[string]interface{}{
			"query":        c.Query,
			"record_type":  c.RecordType,
			"server":       server,
			"answer_count": len(resp.Answer),
			"records":      records,
		})
	} else {
		// Pretty output
		ctx.Output.PrintSuccess(fmt.Sprintf("Found %d record(s):", len(resp.Answer)))
		ctx.Output.Print("")

		for _, rr := range resp.Answer {
			ctx.Output.Print(formatRecord(rr))
		}
	}

	return nil
}

// parseRecord converts a DNS record to a map for JSON output.
func parseRecord(rr mdns.RR) map[string]interface{} {
	header := rr.Header()
	base := map[string]interface{}{
		"name":  header.Name,
		"type":  mdns.TypeToString[header.Rrtype],
		"class": mdns.ClassToString[header.Class],
		"ttl":   header.Ttl,
	}

	// Add type-specific data
	switch v := rr.(type) {
	case *mdns.A:
		base["address"] = v.A.String()
	case *mdns.AAAA:
		base["address"] = v.AAAA.String()
	case *mdns.CNAME:
		base["target"] = v.Target
	case *mdns.MX:
		base["preference"] = v.Preference
		base["exchange"] = v.Mx
	case *mdns.NS:
		base["nameserver"] = v.Ns
	case *mdns.TXT:
		base["text"] = strings.Join(v.Txt, " ")
	case *mdns.SOA:
		base["mname"] = v.Ns
		base["rname"] = v.Mbox
		base["serial"] = v.Serial
		base["refresh"] = v.Refresh
		base["retry"] = v.Retry
		base["expire"] = v.Expire
		base["minimum"] = v.Minttl
	case *mdns.SRV:
		base["priority"] = v.Priority
		base["weight"] = v.Weight
		base["port"] = v.Port
		base["target"] = v.Target
	case *mdns.CAA:
		base["flag"] = v.Flag
		base["tag"] = v.Tag
		base["value"] = v.Value
	}

	return base
}

// formatRecord formats a DNS record for pretty output.
func formatRecord(rr mdns.RR) string {
	switch v := rr.(type) {
	case *mdns.A:
		return fmt.Sprintf("%s\tIN\tA\t%s", v.Header().Name, v.A)
	case *mdns.AAAA:
		return fmt.Sprintf("%s\tIN\tAAAA\t%s", v.Header().Name, v.AAAA)
	case *mdns.CNAME:
		return fmt.Sprintf("%s\tIN\tCNAME\t%s", v.Header().Name, v.Target)
	case *mdns.MX:
		return fmt.Sprintf("%s\tIN\tMX\t%d %s", v.Header().Name, v.Preference, v.Mx)
	case *mdns.NS:
		return fmt.Sprintf("%s\tIN\tNS\t%s", v.Header().Name, v.Ns)
	case *mdns.TXT:
		return fmt.Sprintf("%s\tIN\tTXT\t\"%s\"", v.Header().Name, strings.Join(v.Txt, "\" \""))
	case *mdns.SOA:
		return fmt.Sprintf("%s\tIN\tSOA\t%s %s %d %d %d %d %d",
			v.Header().Name, v.Ns, v.Mbox, v.Serial, v.Refresh, v.Retry, v.Expire, v.Minttl)
	case *mdns.SRV:
		return fmt.Sprintf("%s\tIN\tSRV\t%d %d %d %s",
			v.Header().Name, v.Priority, v.Weight, v.Port, v.Target)
	case *mdns.CAA:
		return fmt.Sprintf("%s\tIN\tCAA\t%d %s \"%s\"",
			v.Header().Name, v.Flag, v.Tag, v.Value)
	default:
		return rr.String()
	}
}
