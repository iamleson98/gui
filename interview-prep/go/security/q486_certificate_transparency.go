// Question #486: Certificate Transparency
// Category: Security | Difficulty: Hard
// Concepts: certificate transparency, CT logs, X.509, mis-issuance
// Description: Validate X.509 certificates against CT logs to detect mis-issuance.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Certificate Transparency
// Implements a security primitive for question #486.
type CertificateTransparency struct {
        key []byte
}

// NewCertificateTransparency creates a new security handler with the given key.
func NewCertificateTransparency(key []byte) *CertificateTransparency {
        return &CertificateTransparency{key: key}
}

// Hash computes a secure hash of the input.
func (s *CertificateTransparency) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *CertificateTransparency) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *CertificateTransparency) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
