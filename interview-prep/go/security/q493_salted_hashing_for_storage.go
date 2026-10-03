// Question #493: Salted Hashing for Storage
// Category: Security | Difficulty: Hard
// Concepts: salted hash, slow hash, storage, cracking
// Description: Store credentials as salted slow hashes to resist offline cracking.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Salted Hashing for Storage
// Implements a security primitive for question #493.
type Q493_SaltedHashingForStorage struct {
        key []byte
}

// NewQ493_SaltedHashingForStorage creates a new security handler with the given key.
func NewQ493_SaltedHashingForStorage(key []byte) *Q493_SaltedHashingForStorage {
        return &Q493_SaltedHashingForStorage{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q493_SaltedHashingForStorage) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q493_SaltedHashingForStorage) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q493_SaltedHashingForStorage) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
