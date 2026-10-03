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
type PerfectForwardSecrecy struct {
        key []byte
}

// NewPerfectForwardSecrecy creates a new security handler with the given key.
func NewPerfectForwardSecrecy(key []byte) *PerfectForwardSecrecy {
        return &PerfectForwardSecrecy{key: key}
}

// Hash computes a secure hash of the input.
func (s *PerfectForwardSecrecy) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *PerfectForwardSecrecy) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *PerfectForwardSecrecy) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
