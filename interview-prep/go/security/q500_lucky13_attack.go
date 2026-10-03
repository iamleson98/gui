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
type Lucky13Attack struct {
        key []byte
}

// NewLucky13Attack creates a new security handler with the given key.
func NewLucky13Attack(key []byte) *Lucky13Attack {
        return &Lucky13Attack{key: key}
}

// Hash computes a secure hash of the input.
func (s *Lucky13Attack) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Lucky13Attack) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Lucky13Attack) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
