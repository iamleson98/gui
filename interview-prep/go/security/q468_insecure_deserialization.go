// Question #468: Insecure Deserialization
// Category: Security | Difficulty: Hard
// Concepts: deserialization, type allowlist, gadget, RCE
// Description: Prevent insecure deserialization by avoiding native formats and enforcing type allowlists.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Insecure Deserialization
// Implements a security primitive for question #468.
type InsecureDeserialization struct {
        key []byte
}

// NewInsecureDeserialization creates a new security handler with the given key.
func NewInsecureDeserialization(key []byte) *InsecureDeserialization {
        return &InsecureDeserialization{key: key}
}

// Hash computes a secure hash of the input.
func (s *InsecureDeserialization) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *InsecureDeserialization) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *InsecureDeserialization) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
