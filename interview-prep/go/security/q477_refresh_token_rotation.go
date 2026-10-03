// Question #477: Refresh Token Rotation
// Category: Security | Difficulty: Hard
// Concepts: refresh token, rotation, reuse detection, theft
// Description: Rotate refresh tokens on use with reuse detection to limit token theft impact.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Refresh Token Rotation
// Implements a security primitive for question #477.
type Q477_RefreshTokenRotation struct {
        key []byte
}

// NewQ477_RefreshTokenRotation creates a new security handler with the given key.
func NewQ477_RefreshTokenRotation(key []byte) *Q477_RefreshTokenRotation {
        return &Q477_RefreshTokenRotation{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q477_RefreshTokenRotation) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q477_RefreshTokenRotation) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q477_RefreshTokenRotation) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
