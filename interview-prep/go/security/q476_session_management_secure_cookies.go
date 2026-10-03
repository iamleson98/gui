// Question #476: Session Management (Secure Cookies)
// Category: Security | Difficulty: Hard
// Concepts: session, HttpOnly, Secure, SameSite
// Description: Manage sessions with HttpOnly, Secure, and SameSite cookie attributes.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Session Management (Secure Cookies)
// Implements a security primitive for question #476.
type SessionManagementSecureCookies struct {
        key []byte
}

// NewSessionManagementSecureCookies creates a new security handler with the given key.
func NewSessionManagementSecureCookies(key []byte) *SessionManagementSecureCookies {
        return &SessionManagementSecureCookies{key: key}
}

// Hash computes a secure hash of the input.
func (s *SessionManagementSecureCookies) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *SessionManagementSecureCookies) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *SessionManagementSecureCookies) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
