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
type Q509_RateLimitingForSecurity struct {
        key []byte
}

// NewQ509_RateLimitingForSecurity creates a new security handler with the given key.
func NewQ509_RateLimitingForSecurity(key []byte) *Q509_RateLimitingForSecurity {
        return &Q509_RateLimitingForSecurity{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q509_RateLimitingForSecurity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q509_RateLimitingForSecurity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q509_RateLimitingForSecurity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
