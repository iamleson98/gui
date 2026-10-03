// Question #466: Command Injection
// Category: Security | Difficulty: Hard
// Concepts: command injection, shell, argument array, escaping
// Description: Prevent command injection by avoiding shell calls and using argument arrays.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Command Injection
// Implements a security primitive for question #466.
type CommandInjection struct {
        key []byte
}

// NewCommandInjection creates a new security handler with the given key.
func NewCommandInjection(key []byte) *CommandInjection {
        return &CommandInjection{key: key}
}

// Hash computes a secure hash of the input.
func (s *CommandInjection) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *CommandInjection) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *CommandInjection) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
