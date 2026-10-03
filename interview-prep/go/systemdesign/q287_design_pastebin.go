// Question #287: Design Pastebin
// Category: System Design | Difficulty: Hard
// Concepts: pastebin, blob storage, expiration, rate limit
// Description: Design a paste-sharing service handling large text blobs, expiration, and rate limits.
package systemdesign

import "sync"

// Design Pastebin
// Implements a system design component for question #287.
type Q287_DesignPastebin struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ287_DesignPastebin creates a new system component.
func NewQ287_DesignPastebin() *Q287_DesignPastebin {
        return &Q287_DesignPastebin{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q287_DesignPastebin) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q287_DesignPastebin) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q287_DesignPastebin) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
