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
type Q498_SideChannelAttacks struct {
        key []byte
}

// NewQ498_SideChannelAttacks creates a new security handler with the given key.
func NewQ498_SideChannelAttacks(key []byte) *Q498_SideChannelAttacks {
        return &Q498_SideChannelAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q498_SideChannelAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q498_SideChannelAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q498_SideChannelAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
