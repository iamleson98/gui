// Question #465: XXE Prevention
// Category: Security | Difficulty: Hard
// Concepts: XXE, DTD, external entity, parser
// Description: Prevent XML external entity attacks by disabling DTDs and external entity resolution.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// XXE Prevention
// Implements a security primitive for question #465.
type Q465_XxePrevention struct {
        key []byte
}

// NewQ465_XxePrevention creates a new security handler with the given key.
func NewQ465_XxePrevention(key []byte) *Q465_XxePrevention {
        return &Q465_XxePrevention{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q465_XxePrevention) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q465_XxePrevention) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q465_XxePrevention) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
