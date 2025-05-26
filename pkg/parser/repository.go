package parser

import (
	"fmt"
	"net/url"
	"regexp"
	"strings"
)

type RepositoryInfo struct {
	Host     string
	Owner    string
	Name     string
	Protocol string
}

var (
	simpleRepoRegex = regexp.MustCompile(`^[a-zA-Z0-9_.-]+$`)
	ownerRepoRegex  = regexp.MustCompile(`^([a-zA-Z0-9_.-]+)/([a-zA-Z0-9_.-]+)$`)
	sshURLRegex     = regexp.MustCompile(`^(?:ssh://)?git@([^:]+):([^/]+)/(.+?)(?:\.git)?$`)
)

func IsSimpleRepo(spec string) bool {
	return simpleRepoRegex.MatchString(spec)
}

func ParseRepository(spec, defaultUsername string) (*RepositoryInfo, error) {
	spec = strings.TrimSpace(spec)

	if simpleRepoRegex.MatchString(spec) {
		if defaultUsername == "" {
			return nil, fmt.Errorf("no username provided for repository '%s'", spec)
		}

		return &RepositoryInfo{
			Host:  "github.com",
			Owner: defaultUsername,
			Name:  spec,
		}, nil
	}

	if matches := ownerRepoRegex.FindStringSubmatch(spec); matches != nil {
		return &RepositoryInfo{
			Host:  "github.com",
			Owner: matches[1],
			Name:  matches[2],
		}, nil
	}

	if strings.HasPrefix(spec, "http://") || strings.HasPrefix(spec, "https://") {
		return parseHTTPURL(spec)
	}

	if strings.HasPrefix(spec, "git@") || strings.HasPrefix(spec, "ssh://git@") {
		return parseSSHURL(spec)
	}

	return nil, fmt.Errorf("invalid repository specification: %s", spec)
}

func parseHTTPURL(urlStr string) (*RepositoryInfo, error) {
	u, err := url.Parse(urlStr)
	if err != nil {
		return nil, fmt.Errorf("invalid URL: %w", err)
	}

	parts := strings.Split(strings.TrimPrefix(u.Path, "/"), "/")
	if len(parts) < 2 {
		return nil, fmt.Errorf("invalid repository URL: %s", urlStr)
	}

	name := strings.TrimSuffix(parts[1], ".git")

	return &RepositoryInfo{
		Host:     u.Host,
		Owner:    parts[0],
		Name:     name,
		Protocol: u.Scheme,
	}, nil
}

func parseSSHURL(urlStr string) (*RepositoryInfo, error) {
	matches := sshURLRegex.FindStringSubmatch(urlStr)
	if matches == nil {
		return nil, fmt.Errorf("invalid SSH URL: %s", urlStr)
	}

	return &RepositoryInfo{
		Host:     matches[1],
		Owner:    matches[2],
		Name:     strings.TrimSuffix(matches[3], ".git"),
		Protocol: "ssh",
	}, nil
}

func (r *RepositoryInfo) CloneURL(protocol string) string {
	switch protocol {
	case "ssh":
		return fmt.Sprintf("git@%s:%s/%s.git", r.Host, r.Owner, r.Name)
	case "https":
		return fmt.Sprintf("https://%s/%s/%s.git", r.Host, r.Owner, r.Name)
	default:
		return fmt.Sprintf("https://%s/%s/%s.git", r.Host, r.Owner, r.Name)
	}
}
