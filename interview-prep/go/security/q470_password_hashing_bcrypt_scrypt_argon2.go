// Question #470: Password Hashing: bcrypt/scrypt/argon2
// Category: Security | Difficulty: Hard
// Concepts: password hashing, bcrypt, scrypt, argon2
// Description: Choose and configure bcrypt, scrypt, and argon2 to slow brute force attacks.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Password Hashing: bcrypt/scrypt/argon2
// Implements a security primitive for question #470.
type Q470_PasswordHashingBcryptScryptArgon2 struct {
        key []byte
}

// NewQ470_PasswordHashingBcryptScryptArgon2 creates a new security handler with the given key.
func NewQ470_PasswordHashingBcryptScryptArgon2(key []byte) *Q470_PasswordHashingBcryptScryptArgon2 {
        return &Q470_PasswordHashingBcryptScryptArgon2{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q470_PasswordHashingBcryptScryptArgon2) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q470_PasswordHashingBcryptScryptArgon2) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q470_PasswordHashingBcryptScryptArgon2) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
