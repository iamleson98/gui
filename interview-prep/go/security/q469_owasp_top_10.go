// Question #469: OWASP Top 10
// Category: Security | Difficulty: Hard
// Concepts: OWASP, Top 10, risk, mitigation
// Description: Map a system's defenses to the OWASP Top 10 risk categories.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// OWASP Top 10
// Implements a security primitive for question #469.
type Q469_OwaspTop10 struct {
        key []byte
}

// NewQ469_OwaspTop10 creates a new security handler with the given key.
func NewQ469_OwaspTop10(key []byte) *Q469_OwaspTop10 {
        return &Q469_OwaspTop10{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q469_OwaspTop10) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q469_OwaspTop10) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q469_OwaspTop10) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
