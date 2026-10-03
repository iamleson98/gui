// Question #474: OpenID Connect (OIDC)
// Category: Security | Difficulty: Hard
// Concepts: OIDC, ID token, OAuth, identity
// Description: Layer OpenID Connect on OAuth 2.0 for authenticated identity via ID tokens.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// OpenID Connect (OIDC)
// Implements a security primitive for question #474.
type OpenidConnectOidc struct {
        key []byte
}

// NewOpenidConnectOidc creates a new security handler with the given key.
func NewOpenidConnectOidc(key []byte) *OpenidConnectOidc {
        return &OpenidConnectOidc{key: key}
}

// Hash computes a secure hash of the input.
func (s *OpenidConnectOidc) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *OpenidConnectOidc) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *OpenidConnectOidc) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
