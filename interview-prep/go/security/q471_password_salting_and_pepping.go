// Question #471: Password Salting and Pepping
// Category: Security | Difficulty: Hard
// Concepts: salt, pepper, precomputed, rainbow
// Description: Use per-password salts and a server-side pepper to defeat precomputed attacks.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Password Salting and Pepping
// Implements a security primitive for question #471.
type PasswordSaltingAndPepping struct {
        key []byte
}

// NewPasswordSaltingAndPepping creates a new security handler with the given key.
func NewPasswordSaltingAndPepping(key []byte) *PasswordSaltingAndPepping {
        return &PasswordSaltingAndPepping{key: key}
}

// Hash computes a secure hash of the input.
func (s *PasswordSaltingAndPepping) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *PasswordSaltingAndPepping) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *PasswordSaltingAndPepping) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
