// Question #487: TLS 1.3 Handshake
// Category: Security | Difficulty: Hard
// Concepts: TLS 1.3, 1-RTT, key share, HKDF
// Description: Walk through the TLS 1.3 1-RTT handshake with key share and HKDF.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// TLS 1.3 Handshake
// Implements a security primitive for question #487.
type Q487_Tls13Handshake struct {
        key []byte
}

// NewQ487_Tls13Handshake creates a new security handler with the given key.
func NewQ487_Tls13Handshake(key []byte) *Q487_Tls13Handshake {
        return &Q487_Tls13Handshake{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q487_Tls13Handshake) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q487_Tls13Handshake) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q487_Tls13Handshake) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
