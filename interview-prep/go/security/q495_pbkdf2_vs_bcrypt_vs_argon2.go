// Question #495: PBKDF2 vs bcrypt vs argon2
// Category: Security | Difficulty: Hard
// Concepts: PBKDF2, bcrypt, argon2, memory-hard
// Description: Compare PBKDF2, bcrypt, and argon2 on CPU/memory hardness and suitability.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// PBKDF2 vs bcrypt vs argon2
// Implements a security primitive for question #495.
type Pbkdf2VsBcryptVsArgon2 struct {
        key []byte
}

// NewPbkdf2VsBcryptVsArgon2 creates a new security handler with the given key.
func NewPbkdf2VsBcryptVsArgon2(key []byte) *Pbkdf2VsBcryptVsArgon2 {
        return &Pbkdf2VsBcryptVsArgon2{key: key}
}

// Hash computes a secure hash of the input.
func (s *Pbkdf2VsBcryptVsArgon2) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Pbkdf2VsBcryptVsArgon2) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Pbkdf2VsBcryptVsArgon2) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
