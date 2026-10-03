// Question #484: ECDH and Curve25519
// Category: Security | Difficulty: Hard
// Concepts: ECDH, Curve25519, key agreement, side channel
// Description: Use ECDH on Curve25519 for fast, secure key agreement resistant to side channels.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// ECDH and Curve25519
// Implements a security primitive for question #484.
type EcdhAndCurve25519 struct {
        key []byte
}

// NewEcdhAndCurve25519 creates a new security handler with the given key.
func NewEcdhAndCurve25519(key []byte) *EcdhAndCurve25519 {
        return &EcdhAndCurve25519{key: key}
}

// Hash computes a secure hash of the input.
func (s *EcdhAndCurve25519) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *EcdhAndCurve25519) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *EcdhAndCurve25519) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
