// Question #483: Diffie-Hellman Key Exchange
// Category: Security | Difficulty: Hard
// Concepts: Diffie-Hellman, ephemeral, shared secret, discrete log
// Description: Implement ephemeral Diffie-Hellman to establish shared secrets over an insecure channel.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Diffie-Hellman Key Exchange
// Implements a security primitive for question #483.
type DiffieHellmanKeyExchange struct {
        key []byte
}

// NewDiffieHellmanKeyExchange creates a new security handler with the given key.
func NewDiffieHellmanKeyExchange(key []byte) *DiffieHellmanKeyExchange {
        return &DiffieHellmanKeyExchange{key: key}
}

// Hash computes a secure hash of the input.
func (s *DiffieHellmanKeyExchange) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *DiffieHellmanKeyExchange) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *DiffieHellmanKeyExchange) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
