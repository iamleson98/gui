// Question #500: Lucky13 Attack
// Category: Security | Difficulty: Hard
// Concepts: Lucky13, TLS CBC, timing, AEAD
// Description: Understand the Lucky13 timing attack on TLS CBC and prefer AEAD ciphers.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Lucky13 Attack
// Implements a security primitive for question #500.
type Q500_Lucky13Attack struct {
        key []byte
}

// NewQ500_Lucky13Attack creates a new security handler with the given key.
func NewQ500_Lucky13Attack(key []byte) *Q500_Lucky13Attack {
        return &Q500_Lucky13Attack{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q500_Lucky13Attack) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q500_Lucky13Attack) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q500_Lucky13Attack) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
