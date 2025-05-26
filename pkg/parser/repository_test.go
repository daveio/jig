package parser

import (
	"testing"
)

func TestParseRepository(t *testing.T) {
	tests := []struct {
		name            string
		spec            string
		defaultUsername string
		want            *RepositoryInfo
		wantErr         bool
	}{
		{
			name:            "simple repo with username",
			spec:            "hubbit",
			defaultUsername: "daveio",
			want: &RepositoryInfo{
				Host:  "github.com",
				Owner: "daveio",
				Name:  "hubbit",
			},
		},
		{
			name:            "simple repo without username",
			spec:            "hubbit",
			defaultUsername: "",
			wantErr:         true,
		},
		{
			name:            "owner/repo format",
			spec:            "daveio/hubbit",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:  "github.com",
				Owner: "daveio",
				Name:  "hubbit",
			},
		},
		{
			name:            "https URL",
			spec:            "https://github.com/daveio/hubbit",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:     "github.com",
				Owner:    "daveio",
				Name:     "hubbit",
				Protocol: "https",
			},
		},
		{
			name:            "https URL with .git",
			spec:            "https://github.com/daveio/hubbit.git",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:     "github.com",
				Owner:    "daveio",
				Name:     "hubbit",
				Protocol: "https",
			},
		},
		{
			name:            "SSH URL",
			spec:            "git@github.com:daveio/hubbit.git",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:     "github.com",
				Owner:    "daveio",
				Name:     "hubbit",
				Protocol: "ssh",
			},
		},
		{
			name:            "SSH URL without .git",
			spec:            "git@github.com:daveio/hubbit",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:     "github.com",
				Owner:    "daveio",
				Name:     "hubbit",
				Protocol: "ssh",
			},
		},
		{
			name:            "SSH URL with ssh:// prefix",
			spec:            "ssh://git@github.com:daveio/hubbit.git",
			defaultUsername: "",
			want: &RepositoryInfo{
				Host:     "github.com",
				Owner:    "daveio",
				Name:     "hubbit",
				Protocol: "ssh",
			},
		},
		{
			name:            "invalid format",
			spec:            "not/a/valid/repo",
			defaultUsername: "",
			wantErr:         true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := ParseRepository(tt.spec, tt.defaultUsername)
			if (err != nil) != tt.wantErr {
				t.Errorf("ParseRepository() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if !tt.wantErr {
				if got.Host != tt.want.Host {
					t.Errorf("ParseRepository() Host = %v, want %v", got.Host, tt.want.Host)
				}
				if got.Owner != tt.want.Owner {
					t.Errorf("ParseRepository() Owner = %v, want %v", got.Owner, tt.want.Owner)
				}
				if got.Name != tt.want.Name {
					t.Errorf("ParseRepository() Name = %v, want %v", got.Name, tt.want.Name)
				}
				if tt.want.Protocol != "" && got.Protocol != tt.want.Protocol {
					t.Errorf("ParseRepository() Protocol = %v, want %v", got.Protocol, tt.want.Protocol)
				}
			}
		})
	}
}

func TestRepositoryInfo_CloneURL(t *testing.T) {
	repo := &RepositoryInfo{
		Host:  "github.com",
		Owner: "daveio",
		Name:  "hubbit",
	}

	tests := []struct {
		name     string
		protocol string
		want     string
	}{
		{
			name:     "SSH protocol",
			protocol: "ssh",
			want:     "git@github.com:daveio/hubbit.git",
		},
		{
			name:     "HTTPS protocol",
			protocol: "https",
			want:     "https://github.com/daveio/hubbit.git",
		},
		{
			name:     "default protocol",
			protocol: "",
			want:     "https://github.com/daveio/hubbit.git",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := repo.CloneURL(tt.protocol); got != tt.want {
				t.Errorf("CloneURL() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIsSimpleRepo(t *testing.T) {
	tests := []struct {
		name string
		spec string
		want bool
	}{
		{
			name: "simple repo name",
			spec: "hubbit",
			want: true,
		},
		{
			name: "repo with dashes",
			spec: "my-repo",
			want: true,
		},
		{
			name: "repo with underscores",
			spec: "my_repo",
			want: true,
		},
		{
			name: "repo with dots",
			spec: "my.repo",
			want: true,
		},
		{
			name: "owner/repo format",
			spec: "daveio/hubbit",
			want: false,
		},
		{
			name: "URL format",
			spec: "https://github.com/daveio/hubbit",
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IsSimpleRepo(tt.spec); got != tt.want {
				t.Errorf("IsSimpleRepo() = %v, want %v", got, tt.want)
			}
		})
	}
}
