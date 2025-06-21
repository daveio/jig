package crypto

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io"

	"github.com/btcsuite/btcutil/base58"
	"golang.org/x/crypto/chacha20poly1305"
	"golang.org/x/crypto/curve25519"
	"lukechampine.com/blake3"
)

// GenerateRandomBytes generates cryptographically secure random bytes.
func GenerateRandomBytes(n int) ([]byte, error) {
	b := make([]byte, n)
	_, err := rand.Read(b)
	if err != nil {
		return nil, fmt.Errorf("generating random bytes: %w", err)
	}
	return b, nil
}

// GenerateRandomHex generates random bytes and returns as hex string.
func GenerateRandomHex(n int) (string, error) {
	bytes, err := GenerateRandomBytes(n)
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(bytes), nil
}

// GenerateKey generates a 32-byte encryption key.
func GenerateKey() ([]byte, error) {
	return GenerateRandomBytes(32)
}

// GenerateKeyBase58 generates a 32-byte key and returns it base58-encoded.
func GenerateKeyBase58() (string, error) {
	key, err := GenerateKey()
	if err != nil {
		return "", err
	}
	return base58.Encode(key), nil
}

// DecodeKeyBase58 decodes a base58-encoded key.
func DecodeKeyBase58(encoded string) ([]byte, error) {
	key := base58.Decode(encoded)
	if len(key) != 32 {
		return nil, fmt.Errorf("invalid key length: expected 32 bytes, got %d", len(key))
	}
	return key, nil
}

// Blake3Hash computes BLAKE3 hash of data (64 bytes).
func Blake3Hash(data []byte) []byte {
	hasher := blake3.New(64, nil)
	_, _ = hasher.Write(data)
	hash := make([]byte, 64)
	hasher.Sum(hash[:0])
	return hash
}

// EncryptBlock encrypts data using ChaCha20Poly1305 with associated data.
func EncryptBlock(key, plaintext, associatedData []byte) (nonce, ciphertext []byte, err error) {
	aead, err := chacha20poly1305.New(key)
	if err != nil {
		return nil, nil, fmt.Errorf("creating cipher: %w", err)
	}

	nonce = make([]byte, aead.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		return nil, nil, fmt.Errorf("generating nonce: %w", err)
	}

	ciphertext = aead.Seal(nil, nonce, plaintext, associatedData)
	return nonce, ciphertext, nil
}

// DecryptBlock decrypts data using ChaCha20Poly1305 with associated data.
func DecryptBlock(key, nonce, ciphertext, associatedData []byte) ([]byte, error) {
	aead, err := chacha20poly1305.New(key)
	if err != nil {
		return nil, fmt.Errorf("creating cipher: %w", err)
	}

	plaintext, err := aead.Open(nil, nonce, ciphertext, associatedData)
	if err != nil {
		return nil, fmt.Errorf("decrypting: %w", err)
	}

	return plaintext, nil
}

// GenerateX25519KeyPair generates an X25519 key pair for WireGuard.
func GenerateX25519KeyPair() (privateKey, publicKey []byte, err error) {
	privateKey, err = GenerateRandomBytes(32)
	if err != nil {
		return nil, nil, fmt.Errorf("generating private key: %w", err)
	}

	// Clamp private key as required by X25519
	privateKey[0] &= 248
	privateKey[31] &= 127
	privateKey[31] |= 64

	publicKey, err = curve25519.X25519(privateKey, curve25519.Basepoint)
	if err != nil {
		return nil, nil, fmt.Errorf("generating public key: %w", err)
	}

	return privateKey, publicKey, nil
}
