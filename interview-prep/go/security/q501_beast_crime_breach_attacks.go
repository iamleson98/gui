// Question #501: BEAST/CRIME/BREACH Attacks
// Category: Security | Difficulty: Hard
// Concepts: BEAST, CRIME, BREACH, compression
// Description: Understand compression and CBC attacks (BEAST, CRIME, BREACH) and their mitigations.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// BEAST/CRIME/BREACH Attacks
// Implements a security primitive for question #501.
type BeastCrimeBreachAttacks struct {
        key []byte
}

// NewBeastCrimeBreachAttacks creates a new security handler with the given key.
func NewBeastCrimeBreachAttacks(key []byte) *BeastCrimeBreachAttacks {
        return &BeastCrimeBreachAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *BeastCrimeBreachAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *BeastCrimeBreachAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *BeastCrimeBreachAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
