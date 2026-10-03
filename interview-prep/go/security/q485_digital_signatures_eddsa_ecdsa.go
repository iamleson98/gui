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
type DigitalSignaturesEddsaEcdsa struct {
        key []byte
}

// NewDigitalSignaturesEddsaEcdsa creates a new security handler with the given key.
func NewDigitalSignaturesEddsaEcdsa(key []byte) *DigitalSignaturesEddsaEcdsa {
        return &DigitalSignaturesEddsaEcdsa{key: key}
}

// Hash computes a secure hash of the input.
func (s *DigitalSignaturesEddsaEcdsa) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *DigitalSignaturesEddsaEcdsa) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *DigitalSignaturesEddsaEcdsa) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
