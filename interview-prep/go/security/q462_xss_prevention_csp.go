// Question #462: XSS Prevention (CSP)
// Category: Security | Difficulty: Hard
// Concepts: XSS, encoding, CSP, sanitization
// Description: Prevent cross-site scripting with output encoding and a Content Security Policy.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// XSS Prevention (CSP)
// Implements a security primitive for question #462.
type XssPreventionCsp struct {
        key []byte
}

// NewXssPreventionCsp creates a new security handler with the given key.
func NewXssPreventionCsp(key []byte) *XssPreventionCsp {
        return &XssPreventionCsp{key: key}
}

// Hash computes a secure hash of the input.
func (s *XssPreventionCsp) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *XssPreventionCsp) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *XssPreventionCsp) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
