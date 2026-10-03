// Question #475: SAML SSO
// Category: Security | Difficulty: Hard
// Concepts: SAML, assertion, SSO, binding
// Description: Implement SAML single sign-on with signed assertions and the POST redirect binding.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// SAML SSO
// Implements a security primitive for question #475.
type Q475_SamlSso struct {
        key []byte
}

// NewQ475_SamlSso creates a new security handler with the given key.
func NewQ475_SamlSso(key []byte) *Q475_SamlSso {
        return &Q475_SamlSso{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q475_SamlSso) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q475_SamlSso) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q475_SamlSso) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
