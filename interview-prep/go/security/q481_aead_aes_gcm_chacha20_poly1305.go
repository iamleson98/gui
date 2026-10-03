// Question #481: AEAD (AES-GCM, ChaCha20-Poly1305)
// Category: Security | Difficulty: Hard
// Concepts: AEAD, AES-GCM, ChaCha20-Poly1305, nonce
// Description: Use authenticated encryption with associated data to provide confidentiality and integrity.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// AEAD (AES-GCM, ChaCha20-Poly1305)
// Implements a security primitive for question #481.
type Q481_AeadAesGcmChacha20Poly1305 struct {
        key []byte
}

// NewQ481_AeadAesGcmChacha20Poly1305 creates a new security handler with the given key.
func NewQ481_AeadAesGcmChacha20Poly1305(key []byte) *Q481_AeadAesGcmChacha20Poly1305 {
        return &Q481_AeadAesGcmChacha20Poly1305{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q481_AeadAesGcmChacha20Poly1305) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q481_AeadAesGcmChacha20Poly1305) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q481_AeadAesGcmChacha20Poly1305) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
