// Question #490: Zero-Trust Architecture
// Category: Security | Difficulty: Hard
// Concepts: zero trust, per-request auth, no implicit trust, policy
// Description: Design a zero-trust architecture authenticating every request without network trust.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Zero-Trust Architecture
// Implements a security primitive for question #490.
type Q490_ZeroTrustArchitecture struct {
        key []byte
}

// NewQ490_ZeroTrustArchitecture creates a new security handler with the given key.
func NewQ490_ZeroTrustArchitecture(key []byte) *Q490_ZeroTrustArchitecture {
        return &Q490_ZeroTrustArchitecture{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q490_ZeroTrustArchitecture) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q490_ZeroTrustArchitecture) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q490_ZeroTrustArchitecture) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
