// Question #480: HMAC (Hash-Based MAC)
// Category: Security | Difficulty: Hard
// Concepts: HMAC, keyed hash, integrity, authentication
// Description: Implement HMAC for message authentication using a keyed hash construction.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// HMAC (Hash-Based MAC)
// Implements a security primitive for question #480.
type HmacHashBasedMac struct {
        key []byte
}

// NewHmacHashBasedMac creates a new security handler with the given key.
func NewHmacHashBasedMac(key []byte) *HmacHashBasedMac {
        return &HmacHashBasedMac{key: key}
}

// Hash computes a secure hash of the input.
func (s *HmacHashBasedMac) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *HmacHashBasedMac) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *HmacHashBasedMac) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
