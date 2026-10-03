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
type Q468_InsecureDeserialization struct {
        key []byte
}

// NewQ468_InsecureDeserialization creates a new security handler with the given key.
func NewQ468_InsecureDeserialization(key []byte) *Q468_InsecureDeserialization {
        return &Q468_InsecureDeserialization{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q468_InsecureDeserialization) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q468_InsecureDeserialization) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q468_InsecureDeserialization) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
