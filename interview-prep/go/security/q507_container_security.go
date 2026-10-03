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
type ContainerSecurity struct {
        key []byte
}

// NewContainerSecurity creates a new security handler with the given key.
func NewContainerSecurity(key []byte) *ContainerSecurity {
        return &ContainerSecurity{key: key}
}

// Hash computes a secure hash of the input.
func (s *ContainerSecurity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *ContainerSecurity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *ContainerSecurity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
