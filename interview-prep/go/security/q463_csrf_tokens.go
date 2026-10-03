// Question #463: CSRF Tokens
// Category: Security | Difficulty: Hard
// Concepts: CSRF, tokens, SameSite, origin
// Description: Prevent cross-site request forgery with anti-CSRF tokens and SameSite cookies.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// CSRF Tokens
// Implements a security primitive for question #463.
type CsrfTokens struct {
        key []byte
}

// NewCsrfTokens creates a new security handler with the given key.
func NewCsrfTokens(key []byte) *CsrfTokens {
        return &CsrfTokens{key: key}
}

// Hash computes a secure hash of the input.
func (s *CsrfTokens) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *CsrfTokens) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *CsrfTokens) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
