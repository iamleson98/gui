// Question #488: Perfect Forward Secrecy
// Category: Security | Difficulty: Hard
// Concepts: PFS, ephemeral, key exchange, compromise
// Description: Achieve perfect forward secrecy with ephemeral key exchange so past traffic resists future key compromise.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Perfect Forward Secrecy
// Implements a security primitive for question #488.
type Q488_PerfectForwardSecrecy struct {
        key []byte
}

// NewQ488_PerfectForwardSecrecy creates a new security handler with the given key.
func NewQ488_PerfectForwardSecrecy(key []byte) *Q488_PerfectForwardSecrecy {
        return &Q488_PerfectForwardSecrecy{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q488_PerfectForwardSecrecy) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q488_PerfectForwardSecrecy) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q488_PerfectForwardSecrecy) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
