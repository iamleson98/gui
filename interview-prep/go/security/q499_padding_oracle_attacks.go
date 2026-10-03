// Question #499: Padding Oracle Attacks
// Category: Security | Difficulty: Hard
// Concepts: padding oracle, CBC, AEAD, MAC
// Description: Prevent padding oracle attacks by using AEAD instead of CBC with MAC-then-encrypt.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Padding Oracle Attacks
// Implements a security primitive for question #499.
type PaddingOracleAttacks struct {
        key []byte
}

// NewPaddingOracleAttacks creates a new security handler with the given key.
func NewPaddingOracleAttacks(key []byte) *PaddingOracleAttacks {
        return &PaddingOracleAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *PaddingOracleAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *PaddingOracleAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *PaddingOracleAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
