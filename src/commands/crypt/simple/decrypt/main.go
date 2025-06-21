package decrypt

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/btcsuite/btcutil/base58"
	"github.com/daveio/belt/src/internal/crypto"
	"github.com/daveio/belt/src/internal/types"
	"github.com/daveio/belt/src/ui"
)

// Cmd represents the decrypt command.
type Cmd struct{}

const delimiter = "\n"

// Run executes the decrypt command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Show key backup warning if not warned
	if !ctx.Config.Crypt.Warned {
		ctx.Output.PrintBox(
			"Remember to back up your encryption key!\nYou can view it with: belt crypt simple key",
			ui.WarningBox,
		)
	}

	// Get encryption key
	keyStr := ctx.Config.Crypt.GetKey()
	if keyStr == "" {
		return fmt.Errorf(
			"no encryption key found. Run 'belt init' or set %s",
			ctx.Config.Crypt.Env,
		)
	}

	// Decode key
	key, err := crypto.DecodeKeyBase58(keyStr)
	if err != nil {
		return fmt.Errorf("invalid encryption key: %w", err)
	}

	// Read all input
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		return fmt.Errorf("reading input: %w", err)
	}

	// Trim any trailing newline
	input = bytes.TrimSpace(input)

	// Split into blocks
	blocks := strings.Split(string(input), delimiter)

	// Decrypt each block
	for i, block := range blocks {
		if block == "" {
			continue
		}

		// Decode from base58
		data := base58.Decode(block)
		if len(data) < 12+64 { // nonce + hash minimum
			return fmt.Errorf("block %d: invalid ciphertext", i+1)
		}

		// Extract components
		nonce := data[:12]
		hash := data[len(data)-64:]
		ciphertext := data[12 : len(data)-64]

		// Decrypt
		plaintext, err := crypto.DecryptBlock(key, nonce, ciphertext, hash)
		if err != nil {
			return fmt.Errorf("block %d: decryption failed: %w", i+1, err)
		}

		// Verify hash
		computedHash := crypto.Blake3Hash(plaintext)
		if !bytes.Equal(hash, computedHash) {
			return fmt.Errorf("block %d: hash mismatch - data may be corrupted", i+1)
		}

		// Output plaintext
		if _, err := os.Stdout.Write(plaintext); err != nil {
			return fmt.Errorf("writing output: %w", err)
		}
	}

	return nil
}
