// Question #492: Merkle Tree for Integrity
// Category: Security | Difficulty: Hard
// Concepts: Merkle tree, integrity, proof, tamper detection
// Description: Use Merkle trees to verify integrity of large data sets with compact proofs.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Merkle Tree for Integrity
// Implements a security primitive for question #492.
type MerkleTreeForIntegrity struct {
        key []byte
}

// NewMerkleTreeForIntegrity creates a new security handler with the given key.
func NewMerkleTreeForIntegrity(key []byte) *MerkleTreeForIntegrity {
        return &MerkleTreeForIntegrity{key: key}
}

// Hash computes a secure hash of the input.
func (s *MerkleTreeForIntegrity) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *MerkleTreeForIntegrity) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *MerkleTreeForIntegrity) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
