// Question #479: Passkeys (WebAuthn)
// Category: Security | Difficulty: Hard
// Concepts: passkeys, WebAuthn, attestation, challenge
// Description: Implement passkeys using WebAuthn with authenticator attestation and challenge-response.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Passkeys (WebAuthn)
// Implements a security primitive for question #479.
type Q479_PasskeysWebauthn struct {
        key []byte
}

// NewQ479_PasskeysWebauthn creates a new security handler with the given key.
func NewQ479_PasskeysWebauthn(key []byte) *Q479_PasskeysWebauthn {
        return &Q479_PasskeysWebauthn{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q479_PasskeysWebauthn) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q479_PasskeysWebauthn) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q479_PasskeysWebauthn) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
