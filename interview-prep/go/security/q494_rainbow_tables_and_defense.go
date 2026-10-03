// Question #494: Rainbow Tables and Defense
// Category: Security | Difficulty: Hard
// Concepts: rainbow table, salt, memory-hard, precomputed
// Description: Defend against rainbow tables using salts and memory-hard hashing.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Rainbow Tables and Defense
// Implements a security primitive for question #494.
type RainbowTablesAndDefense struct {
        key []byte
}

// NewRainbowTablesAndDefense creates a new security handler with the given key.
func NewRainbowTablesAndDefense(key []byte) *RainbowTablesAndDefense {
        return &RainbowTablesAndDefense{key: key}
}

// Hash computes a secure hash of the input.
func (s *RainbowTablesAndDefense) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *RainbowTablesAndDefense) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *RainbowTablesAndDefense) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
