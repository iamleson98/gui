// Question #485: Digital Signatures (EdDSA, ECDSA)
// Category: Security | Difficulty: Hard
// Concepts: EdDSA, ECDSA, nonce, canonical
// Description: Sign messages with EdDSA or ECDSA, noting canonical signatures and nonce risks.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Digital Signatures (EdDSA, ECDSA)
// Implements a security primitive for question #485.
type Q485_DigitalSignaturesEddsaEcdsa struct {
        key []byte
}

// NewQ485_DigitalSignaturesEddsaEcdsa creates a new security handler with the given key.
func NewQ485_DigitalSignaturesEddsaEcdsa(key []byte) *Q485_DigitalSignaturesEddsaEcdsa {
        return &Q485_DigitalSignaturesEddsaEcdsa{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q485_DigitalSignaturesEddsaEcdsa) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q485_DigitalSignaturesEddsaEcdsa) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q485_DigitalSignaturesEddsaEcdsa) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
