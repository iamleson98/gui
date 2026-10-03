// Question #498: Side-Channel Attacks
// Category: Security | Difficulty: Hard
// Concepts: side channel, cache, power, EM
// Description: Defend against cache, power, and EM side channels in sensitive code.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Side-Channel Attacks
// Implements a security primitive for question #498.
type SideChannelAttacks struct {
        key []byte
}

// NewSideChannelAttacks creates a new security handler with the given key.
func NewSideChannelAttacks(key []byte) *SideChannelAttacks {
        return &SideChannelAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *SideChannelAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *SideChannelAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *SideChannelAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
