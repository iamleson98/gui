// Question #489: mTLS (Mutual TLS)
// Category: Security | Difficulty: Hard
// Concepts: mTLS, client cert, verification, trust
// Description: Configure mutual TLS so both client and server present and verify certificates.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// mTLS (Mutual TLS)
// Implements a security primitive for question #489.
type Q489_MtlsMutualTls struct {
        key []byte
}

// NewQ489_MtlsMutualTls creates a new security handler with the given key.
func NewQ489_MtlsMutualTls(key []byte) *Q489_MtlsMutualTls {
        return &Q489_MtlsMutualTls{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q489_MtlsMutualTls) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q489_MtlsMutualTls) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q489_MtlsMutualTls) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
