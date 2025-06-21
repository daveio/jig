package pw

import (
	"crypto/rand"
	"fmt"
	"math/big"

	"github.com/daveio/belt/src/internal/types"
)

// Cmd represents the random password command.
type Cmd struct {
	Length int `arg:"" optional:"" default:"16" help:"Password length (default: 16)."`
}

const (
	digits      = "0123456789"
	letters     = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	punctuation = "-_.@#$%&*+=:"
)

// Run executes the random password command.
func (c *Cmd) Run(ctx *types.Context) error {
	// Validate length
	if c.Length < 3 {
		return fmt.Errorf("password length must be at least 3")
	}

	// Generate password
	password, err := generatePassword(c.Length)
	if err != nil {
		return fmt.Errorf("generating password: %w", err)
	}

	// Output based on format
	if ctx.Config.Output.Format == "json" {
		ctx.Output.PrintData(map[string]interface{}{
			"password": password,
			"length":   c.Length,
		})
	} else {
		// Direct output to stdout for scripting
		fmt.Println(password)
	}

	return nil
}

// generatePassword creates a password with specific requirements.
func generatePassword(length int) (string, error) {
	// Build character set
	fullAlphabet := letters + digits + punctuation

	// Create password array
	password := make([]byte, length)

	// First character: always a digit
	digitIndex, err := randInt(len(digits))
	if err != nil {
		return "", err
	}
	password[0] = digits[digitIndex]

	// Last character: always punctuation
	punctIndex, err := randInt(len(punctuation))
	if err != nil {
		return "", err
	}
	password[length-1] = punctuation[punctIndex]

	// Middle characters: random from full alphabet
	for i := 1; i < length-1; i++ {
		charIndex, err := randInt(len(fullAlphabet))
		if err != nil {
			return "", err
		}
		password[i] = fullAlphabet[charIndex]
	}

	return string(password), nil
}

// randInt returns a random integer in [0, max).
func randInt(max int) (int, error) {
	n, err := rand.Int(rand.Reader, big.NewInt(int64(max)))
	if err != nil {
		return 0, err
	}
	return int(n.Int64()), nil
}
