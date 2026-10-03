// Question #510: Honeypots and Intrusion Detection
// Category: Security | Difficulty: Hard
// Concepts: honeypot, IDS, detection, deception
// Description: Deploy honeypots and IDS to detect and analyze attackers.
package security

import (
        "crypto/hmac"
        "crypto/sha256"
        "crypto/subtle"
        "encoding/hex"
)

// Honeypots and Intrusion Detection
// Implements a security primitive for question #510.
type Q510_HoneypotsAndIntrusionDetection struct {
        key []byte
}

// NewQ510_HoneypotsAndIntrusionDetection creates a new security handler with the given key.
func NewQ510_HoneypotsAndIntrusionDetection(key []byte) *Q510_HoneypotsAndIntrusionDetection {
        return &Q510_HoneypotsAndIntrusionDetection{key: key}
}

// Hash computes a secure hash of the input.
func (s *Q510_HoneypotsAndIntrusionDetection) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *Q510_HoneypotsAndIntrusionDetection) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *Q510_HoneypotsAndIntrusionDetection) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
