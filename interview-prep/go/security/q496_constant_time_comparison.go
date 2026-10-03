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
type ConstantTimeComparison struct {
        key []byte
}

// NewConstantTimeComparison creates a new security handler with the given key.
func NewConstantTimeComparison(key []byte) *ConstantTimeComparison {
        return &ConstantTimeComparison{key: key}
}

// Hash computes a secure hash of the input.
func (s *ConstantTimeComparison) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *ConstantTimeComparison) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *ConstantTimeComparison) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
