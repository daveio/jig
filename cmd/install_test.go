package cmd

import (
	"github.com/spf13/cobra"
	"testing"
)

func TestInstallRun(t *testing.T) {
	type args struct {
		cmd  *cobra.Command
		args []string
	}
	var tests = []struct {
		name string
		args args
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
		})
	}
}
