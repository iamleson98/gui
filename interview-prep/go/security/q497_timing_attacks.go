// Question #497: Timing Attacks
// Category: Security | Difficulty: Hard
// Concepts: timing attack, side channel, leak, secret
// Description: Reason about timing side channels that leak secrets through response time variation.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Timing Attacks
// Implements a security primitive for question #497.
type TimingAttacks struct {
        key []byte
}

// NewTimingAttacks creates a new security handler with the given key.
func NewTimingAttacks(key []byte) *TimingAttacks {
        return &TimingAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *TimingAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *TimingAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *TimingAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
