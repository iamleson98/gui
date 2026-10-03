// Question #505: Principle of Least Privilege
// Category: Security | Difficulty: Hard
// Concepts: least privilege, scoping, minimization, principle
// Description: Apply least privilege by granting the minimum scopes needed for a task.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Principle of Least Privilege
// Implements a security primitive for question #505.
type PrincipleOfLeastPrivilege struct {
        key []byte
}

// NewPrincipleOfLeastPrivilege creates a new security handler with the given key.
func NewPrincipleOfLeastPrivilege(key []byte) *PrincipleOfLeastPrivilege {
        return &PrincipleOfLeastPrivilege{key: key}
}

// Hash computes a secure hash of the input.
func (s *PrincipleOfLeastPrivilege) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *PrincipleOfLeastPrivilege) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *PrincipleOfLeastPrivilege) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
