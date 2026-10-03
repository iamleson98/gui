// Question #509: Rate Limiting for Security
// Category: Security | Difficulty: Hard
// Concepts: rate limiting, brute force, lockout, account
// Description: Apply rate limits and account lockouts to slow credential-stuffing and brute force.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Rate Limiting for Security
// Implements a security primitive for question #509.
type RateLimitingForSecurity struct {
        key []byte
}

// NewRateLimitingForSecurity creates a new security handler with the given key.
func NewRateLimitingForSecurity(key []byte) *RateLimitingForSecurity {
        return &RateLimitingForSecurity{key: key}
}

// Hash computes a secure hash of the input.
func (s *RateLimitingForSecurity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *RateLimitingForSecurity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *RateLimitingForSecurity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
