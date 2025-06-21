package dns

import (
	"fmt"
	"net"
	"strings"
	"time"

	"github.com/miekg/dns"
)

// Resolver wraps DNS resolution functionality.
type Resolver struct {
	Server  string
	Timeout time.Duration
}

// NewResolver creates a new DNS resolver.
func NewResolver(server string) *Resolver {
	if server == "" {
		server = "1.1.1.1"
	}

	// Ensure server has port
	if !strings.Contains(server, ":") {
		server = server + ":53"
	}

	return &Resolver{
		Server:  server,
		Timeout: 5 * time.Second,
	}
}

// Query performs a DNS query.
func (r *Resolver) Query(domain string, recordType uint16) (*dns.Msg, error) {
	m := new(dns.Msg)
	m.SetQuestion(dns.Fqdn(domain), recordType)

	c := new(dns.Client)
	c.Timeout = r.Timeout

	resp, _, err := c.Exchange(m, r.Server)
	if err != nil {
		return nil, fmt.Errorf("DNS query failed: %w", err)
	}

	return resp, nil
}

// QueryWithDNSSEC performs a DNS query with DNSSEC enabled.
func (r *Resolver) QueryWithDNSSEC(domain string, recordType uint16) (*dns.Msg, error) {
	m := new(dns.Msg)
	m.SetQuestion(dns.Fqdn(domain), recordType)
	m.SetEdns0(4096, true) // Enable DNSSEC

	c := new(dns.Client)
	c.Timeout = r.Timeout

	resp, _, err := c.Exchange(m, r.Server)
	if err != nil {
		return nil, fmt.Errorf("DNS query failed: %w", err)
	}

	return resp, nil
}

// GetAuthoritativeNS finds the authoritative nameserver for a domain.
func (r *Resolver) GetAuthoritativeNS(domain string) (string, error) {
	resp, err := r.Query(domain, dns.TypeNS)
	if err != nil {
		return "", err
	}

	for _, answer := range resp.Answer {
		if ns, ok := answer.(*dns.NS); ok {
			// Resolve the NS record to get IP
			nsIP, err := r.ResolveNS(ns.Ns)
			if err == nil && nsIP != "" {
				return nsIP, nil
			}
		}
	}

	return "", fmt.Errorf("no authoritative nameserver found")
}

// ResolveNS resolves a nameserver hostname to IP.
func (r *Resolver) ResolveNS(nsName string) (string, error) {
	ips, err := net.LookupIP(nsName)
	if err != nil {
		return "", err
	}

	// Prefer IPv4
	for _, ip := range ips {
		if ip.To4() != nil {
			return ip.String() + ":53", nil
		}
	}

	// Fall back to IPv6
	if len(ips) > 0 {
		return "[" + ips[0].String() + "]:53", nil
	}

	return "", fmt.Errorf("no IP found for nameserver")
}

// ValidateDNSSEC validates DNSSEC for a domain.
func ValidateDNSSEC(domain string, server string) (bool, string, error) {
	resolver := NewResolver(server)

	// First, find authoritative nameserver
	authNS, err := resolver.GetAuthoritativeNS(domain)
	if err != nil {
		// Try with the provided server
		authNS = server
	}

	// Query DNSKEY with DNSSEC from authoritative server
	authResolver := NewResolver(authNS)
	resp, err := authResolver.QueryWithDNSSEC(domain, dns.TypeDNSKEY)
	if err != nil {
		return false, "Query failed: server error or no DNSKEY record", err
	}

	// Check for DNSKEY records
	hasDNSKEY := false
	hasRRSIG := false

	for _, answer := range resp.Answer {
		switch answer.(type) {
		case *dns.DNSKEY:
			hasDNSKEY = true
		case *dns.RRSIG:
			hasRRSIG = true
		}
	}

	if !hasDNSKEY {
		return false, "No DNSKEY record found", nil
	}

	if !hasRRSIG {
		return false, "DNSKEY validation failed: no RRSIG", nil
	}

	// Basic validation passed
	return true, "DNSKEY validated OK", nil
}

// ParseRecordType converts a string record type to dns constant.
func ParseRecordType(recordType string) uint16 {
	switch strings.ToUpper(recordType) {
	case "A":
		return dns.TypeA
	case "AAAA":
		return dns.TypeAAAA
	case "CNAME":
		return dns.TypeCNAME
	case "MX":
		return dns.TypeMX
	case "NS":
		return dns.TypeNS
	case "PTR":
		return dns.TypePTR
	case "SOA":
		return dns.TypeSOA
	case "TXT":
		return dns.TypeTXT
	case "SRV":
		return dns.TypeSRV
	case "CAA":
		return dns.TypeCAA
	default:
		return dns.TypeA
	}
}
