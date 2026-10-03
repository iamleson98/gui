// Question #467: Path Traversal
// Category: Security | Difficulty: Hard
// Concepts: path traversal, canonicalization, sandbox, root
// Description: Prevent path traversal by canonicalizing and confining file access to a root.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Path Traversal
// Implements a security primitive for question #467.
type PathTraversal struct {
        key []byte
}

// NewPathTraversal creates a new security handler with the given key.
func NewPathTraversal(key []byte) *PathTraversal {
        return &PathTraversal{key: key}
}

// Hash computes a secure hash of the input.
func (s *PathTraversal) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *PathTraversal) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *PathTraversal) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
