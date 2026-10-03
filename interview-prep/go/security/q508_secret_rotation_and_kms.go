// Question #508: Secret Rotation and KMS
// Category: Security | Difficulty: Hard
// Concepts: secret rotation, KMS, envelope encryption, audit
// Description: Rotate secrets via a KMS with envelope encryption and audit logging.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Secret Rotation and KMS
// Implements a security primitive for question #508.
type SecretRotationAndKms struct {
        key []byte
}

// NewSecretRotationAndKms creates a new security handler with the given key.
func NewSecretRotationAndKms(key []byte) *SecretRotationAndKms {
        return &SecretRotationAndKms{key: key}
}

// Hash computes a secure hash of the input.
func (s *SecretRotationAndKms) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *SecretRotationAndKms) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *SecretRotationAndKms) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
