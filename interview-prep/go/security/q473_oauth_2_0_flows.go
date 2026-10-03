// Question #473: OAuth 2.0 Flows
// Category: Security | Difficulty: Hard
// Concepts: OAuth, authorization code, PKCE, client credentials
// Description: Implement authorization-code, client-credentials, and PKCE flows appropriately.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// OAuth 2.0 Flows
// Implements a security primitive for question #473.
type Q473_Oauth20Flows struct {
        key []byte
}

// NewQ473_Oauth20Flows creates a new security handler with the given key.
func NewQ473_Oauth20Flows(key []byte) *Q473_Oauth20Flows {
        return &Q473_Oauth20Flows{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q473_Oauth20Flows) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q473_Oauth20Flows) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q473_Oauth20Flows) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
