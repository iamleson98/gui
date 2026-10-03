// Question #472: JWT Design and Pitfalls
// Category: Security | Difficulty: Hard
// Concepts: JWT, alg, expiry, signature
// Description: Design JWTs securely, avoiding alg=none, weak keys, and missing expiry validation.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// JWT Design and Pitfalls
// Implements a security primitive for question #472.
type JwtDesignAndPitfalls struct {
        key []byte
}

// NewJwtDesignAndPitfalls creates a new security handler with the given key.
func NewJwtDesignAndPitfalls(key []byte) *JwtDesignAndPitfalls {
        return &JwtDesignAndPitfalls{key: key}
}

// Hash computes a secure hash of the input.
func (s *JwtDesignAndPitfalls) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *JwtDesignAndPitfalls) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *JwtDesignAndPitfalls) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
