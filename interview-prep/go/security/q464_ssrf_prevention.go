// Question #464: SSRF Prevention
// Category: Security | Difficulty: Hard
// Concepts: SSRF, allowlist, egress, metadata
// Description: Prevent server-side request forgery by validating and restricting outbound destinations.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// SSRF Prevention
// Implements a security primitive for question #464.
type Q464_SsrfPrevention struct {
        key []byte
}

// NewQ464_SsrfPrevention creates a new security handler with the given key.
func NewQ464_SsrfPrevention(key []byte) *Q464_SsrfPrevention {
        return &Q464_SsrfPrevention{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q464_SsrfPrevention) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q464_SsrfPrevention) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q464_SsrfPrevention) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
