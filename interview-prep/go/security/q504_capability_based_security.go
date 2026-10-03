// Question #504: Capability-Based Security
// Category: Security | Difficulty: Hard
// Concepts: capabilities, unforgeable, delegation, ACL
// Description: Design authorization around unforgeable capabilities rather than identity-based ACLs.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Capability-Based Security
// Implements a security primitive for question #504.
type CapabilityBasedSecurity struct {
        key []byte
}

// NewCapabilityBasedSecurity creates a new security handler with the given key.
func NewCapabilityBasedSecurity(key []byte) *CapabilityBasedSecurity {
        return &CapabilityBasedSecurity{key: key}
}

// Hash computes a secure hash of the input.
func (s *CapabilityBasedSecurity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *CapabilityBasedSecurity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *CapabilityBasedSecurity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
