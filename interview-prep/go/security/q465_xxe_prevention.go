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
type XxePrevention struct {
        key []byte
}

// NewXxePrevention creates a new security handler with the given key.
func NewXxePrevention(key []byte) *XxePrevention {
        return &XxePrevention{key: key}
}

// Hash computes a secure hash of the input.
func (s *XxePrevention) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *XxePrevention) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *XxePrevention) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
