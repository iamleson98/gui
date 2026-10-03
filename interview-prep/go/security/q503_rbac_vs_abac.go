// Question #503: RBAC vs ABAC
// Category: Security | Difficulty: Hard
// Concepts: RBAC, ABAC, authorization, policy
// Description: Choose between role-based and attribute-based access control for authorization.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// RBAC vs ABAC
// Implements a security primitive for question #503.
type Q503_RbacVsAbac struct {
        key []byte
}

// NewQ503_RbacVsAbac creates a new security handler with the given key.
func NewQ503_RbacVsAbac(key []byte) *Q503_RbacVsAbac {
        return &Q503_RbacVsAbac{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q503_RbacVsAbac) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q503_RbacVsAbac) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q503_RbacVsAbac) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
