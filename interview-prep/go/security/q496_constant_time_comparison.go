// Question #496: Constant-Time Comparison
// Category: Security | Difficulty: Hard
// Concepts: constant time, timing leak, comparison, secret
// Description: Implement constant-time comparison to avoid leaking equality via timing.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Constant-Time Comparison
// Implements a security primitive for question #496.
type Q496_ConstantTimeComparison struct {
        key []byte
}

// NewQ496_ConstantTimeComparison creates a new security handler with the given key.
func NewQ496_ConstantTimeComparison(key []byte) *Q496_ConstantTimeComparison {
        return &Q496_ConstantTimeComparison{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q496_ConstantTimeComparison) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q496_ConstantTimeComparison) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q496_ConstantTimeComparison) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
