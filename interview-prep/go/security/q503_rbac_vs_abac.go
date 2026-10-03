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
type RbacVsAbac struct {
        key []byte
}

// NewRbacVsAbac creates a new security handler with the given key.
func NewRbacVsAbac(key []byte) *RbacVsAbac {
        return &RbacVsAbac{key: key}
}

// Hash computes a secure hash of the input.
func (s *RbacVsAbac) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *RbacVsAbac) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *RbacVsAbac) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
