package encrypt

import (
	"bytes"
	"fmt"
	"io"
	"os"

	"github.com/btcsuite/btcutil/base58"
	"github.com/daveio/belt/src/internal/crypto"
	"github.com/daveio/belt/src/internal/types"
	"github.com/daveio/belt/src/ui"
)

// Cmd represents the encrypt command.
type Cmd struct{}

const (
	blockSize = 1024
	delimiter = "\n"
)

// Run executes the encrypt command.
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

	// Read and encrypt data in blocks
	buffer := make([]byte, blockSize)
	firstBlock := true

	for {
		n, err := io.ReadFull(os.Stdin, buffer)
		if err == io.EOF && n == 0 {
			break
		}
		if err != nil && err != io.EOF && err != io.ErrUnexpectedEOF {
			return fmt.Errorf("reading input: %w", err)
		}

		// Process the block (may be less than blockSize for last block)
		plaintext := buffer[:n]

		// Compute hash of plaintext
		hash := crypto.Blake3Hash(plaintext)

		// Encrypt with hash as associated data
		nonce, ciphertext, err := crypto.EncryptBlock(key, plaintext, hash)
		if err != nil {
			return fmt.Errorf("encrypting block: %w", err)
		}

		// Construct output: nonce + ciphertext + hash
		var output bytes.Buffer
		output.Write(nonce)
		output.Write(ciphertext)
		output.Write(hash)

		// Encode to base58
		encoded := base58.Encode(output.Bytes())

		// Write block with delimiter
		if !firstBlock {
			fmt.Print(delimiter)
		}
		fmt.Print(encoded)
		firstBlock = false

		// If we read less than blockSize, we're done
		if n < blockSize {
			break
		}
	}

	// Final newline for cleaner output
	fmt.Println()

	return nil
}
