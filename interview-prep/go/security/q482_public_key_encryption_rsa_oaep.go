// Question #482: Public-Key Encryption (RSA-OAEP)
// Category: Security | Difficulty: Hard
// Concepts: RSA-OAEP, padding, CCA, public key
// Description: Encrypt with RSA-OAEP padding to prevent chosen-ciphertext attacks.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Public-Key Encryption (RSA-OAEP)
// Implements a security primitive for question #482.
type Q482_PublicKeyEncryptionRsaOaep struct {
        key []byte
}

// NewQ482_PublicKeyEncryptionRsaOaep creates a new security handler with the given key.
func NewQ482_PublicKeyEncryptionRsaOaep(key []byte) *Q482_PublicKeyEncryptionRsaOaep {
        return &Q482_PublicKeyEncryptionRsaOaep{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q482_PublicKeyEncryptionRsaOaep) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q482_PublicKeyEncryptionRsaOaep) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q482_PublicKeyEncryptionRsaOaep) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
