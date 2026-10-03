// Question #478: MFA / TOTP
// Category: Security | Difficulty: Hard
// Concepts: MFA, TOTP, RFC 6238, HMAC
// Description: Implement time-based one-time passwords (RFC 6238) for multi-factor authentication.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// MFA / TOTP
// Implements a security primitive for question #478.
type MfaTotp struct {
        key []byte
}

// NewMfaTotp creates a new security handler with the given key.
func NewMfaTotp(key []byte) *MfaTotp {
        return &MfaTotp{key: key}
}

// Hash computes a secure hash of the input.
func (s *MfaTotp) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *MfaTotp) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *MfaTotp) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
