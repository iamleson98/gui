// Question #491: Hashing vs Encryption vs Encoding
// Category: Security | Difficulty: Hard
// Concepts: hashing, encryption, encoding, purpose
// Description: Distinguish hashing, encryption, and encoding and pick the right tool for each task.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Hashing vs Encryption vs Encoding
// Implements a security primitive for question #491.
type Q491_HashingVsEncryptionVsEncoding struct {
        key []byte
}

// NewQ491_HashingVsEncryptionVsEncoding creates a new security handler with the given key.
func NewQ491_HashingVsEncryptionVsEncoding(key []byte) *Q491_HashingVsEncryptionVsEncoding {
        return &Q491_HashingVsEncryptionVsEncoding{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q491_HashingVsEncryptionVsEncoding) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q491_HashingVsEncryptionVsEncoding) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q491_HashingVsEncryptionVsEncoding) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
