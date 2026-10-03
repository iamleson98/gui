// Question #461: SQL Injection Prevention
// Category: Security | Difficulty: Hard
// Concepts: SQL injection, parameterized, validation, ORM
// Description: Prevent SQL injection using parameterized queries and strict input validation.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// SQL Injection Prevention
// Implements a security primitive for question #461.
type Q461_SqlInjectionPrevention struct {
        key []byte
}

// NewQ461_SqlInjectionPrevention creates a new security handler with the given key.
func NewQ461_SqlInjectionPrevention(key []byte) *Q461_SqlInjectionPrevention {
        return &Q461_SqlInjectionPrevention{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q461_SqlInjectionPrevention) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q461_SqlInjectionPrevention) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q461_SqlInjectionPrevention) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
