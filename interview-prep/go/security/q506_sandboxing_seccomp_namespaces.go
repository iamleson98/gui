// Question #506: Sandboxing (seccomp, namespaces)
// Category: Security | Difficulty: Hard
// Concepts: sandbox, seccomp, namespaces, syscall filter
// Description: Sandbox untrusted code with seccomp filters and Linux namespaces.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Sandboxing (seccomp, namespaces)
// Implements a security primitive for question #506.
type Q506_SandboxingSeccompNamespaces struct {
        key []byte
}

// NewQ506_SandboxingSeccompNamespaces creates a new security handler with the given key.
func NewQ506_SandboxingSeccompNamespaces(key []byte) *Q506_SandboxingSeccompNamespaces {
        return &Q506_SandboxingSeccompNamespaces{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q506_SandboxingSeccompNamespaces) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q506_SandboxingSeccompNamespaces) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q506_SandboxingSeccompNamespaces) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
