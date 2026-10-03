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
type HoneypotsAndIntrusionDetection struct {
        key []byte
}

// NewHoneypotsAndIntrusionDetection creates a new security handler with the given key.
func NewHoneypotsAndIntrusionDetection(key []byte) *HoneypotsAndIntrusionDetection {
        return &HoneypotsAndIntrusionDetection{key: key}
}

// Hash computes a secure hash of the input.
func (s *HoneypotsAndIntrusionDetection) Hash(data []byte) string {
        h := sha256.Sum256(data)
        return hex.EncodeToString(h[:])
}

// Verify performs a constant-time comparison.
func (s *HoneypotsAndIntrusionDetection) Verify(a, b []byte) bool {
        return subtle.ConstantTimeCompare(a, b) == 1
}

// HMAC computes an HMAC-SHA256 of the data.
func (s *HoneypotsAndIntrusionDetection) HMAC(data []byte) []byte {
        mac := hmac.New(sha256.New, s.key)
        mac.Write(data)
        return mac.Sum(nil)
}
