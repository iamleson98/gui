// Question #502: Replay Attacks and Nonces
// Category: Security | Difficulty: Hard
// Concepts: replay, nonce, timestamp, sequence
// Description: Defend against replay using nonces, timestamps, and sequence numbers.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Replay Attacks and Nonces
// Implements a security primitive for question #502.
type Q502_ReplayAttacksAndNonces struct {
        key []byte
}

// NewQ502_ReplayAttacksAndNonces creates a new security handler with the given key.
func NewQ502_ReplayAttacksAndNonces(key []byte) *Q502_ReplayAttacksAndNonces {
        return &Q502_ReplayAttacksAndNonces{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q502_ReplayAttacksAndNonces) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q502_ReplayAttacksAndNonces) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q502_ReplayAttacksAndNonces) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
