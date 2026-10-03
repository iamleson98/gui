// Question #507: Container Security
// Category: Security | Difficulty: Hard
// Concepts: container, capabilities, read-only, image
// Description: Harden containers with reduced capabilities, read-only roots, and minimal images.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Container Security
// Implements a security primitive for question #507.
type Q507_ContainerSecurity struct {
        key []byte
}

// NewQ507_ContainerSecurity creates a new security handler with the given key.
func NewQ507_ContainerSecurity(key []byte) *Q507_ContainerSecurity {
        return &Q507_ContainerSecurity{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q507_ContainerSecurity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q507_ContainerSecurity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q507_ContainerSecurity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
