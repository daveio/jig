package wireguard

import (
	"encoding/base64"
	"fmt"

	"github.com/daveio/belt/src/internal/crypto"
	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the wireguard key generation command.
type Cmd struct{}

// Run executes the wireguard command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Generate X25519 keypair
	privateKey, publicKey, err := crypto.GenerateX25519KeyPair()
	if err != nil {
		return fmt.Errorf("generating keypair: %w", err)
	}

	// Encode to base64 (standard for WireGuard)
	privateKeyB64 := base64.StdEncoding.EncodeToString(privateKey)
	publicKeyB64 := base64.StdEncoding.EncodeToString(publicKey)

	// Output based on format
	if ctx.Config.Output.Format == "json" {
		ctx.Output.PrintData(map[string]interface{}{
			"private_key": privateKeyB64,
			"public_key":  publicKeyB64,
		})
	} else {
		// Pretty output
		ctx.Output.PrintKeyValue("Private key", privateKeyB64)
		ctx.Output.PrintKeyValue("Public key", publicKeyB64)
	}

	return nil
}
