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
type Q501_BeastCrimeBreachAttacks struct {
        key []byte
}

// NewQ501_BeastCrimeBreachAttacks creates a new security handler with the given key.
func NewQ501_BeastCrimeBreachAttacks(key []byte) *Q501_BeastCrimeBreachAttacks {
        return &Q501_BeastCrimeBreachAttacks{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q501_BeastCrimeBreachAttacks) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q501_BeastCrimeBreachAttacks) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q501_BeastCrimeBreachAttacks) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
