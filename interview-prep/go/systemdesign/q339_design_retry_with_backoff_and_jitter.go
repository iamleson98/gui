// Question #339: Design Retry with Backoff and Jitter
// Category: System Design | Difficulty: Hard
// Concepts: retry, exponential backoff, jitter, deadline
// Description: Design retries with exponential backoff, jitter, and deadline propagation.
package systemdesign

import "sync"

// Design Retry with Backoff and Jitter
// Implements a system design component for question #339.
type DesignRetryWithBackoffAndJitter struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignRetryWithBackoffAndJitter creates a new system component.
func NewDesignRetryWithBackoffAndJitter() *DesignRetryWithBackoffAndJitter {
        return &DesignRetryWithBackoffAndJitter{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignRetryWithBackoffAndJitter) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignRetryWithBackoffAndJitter) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignRetryWithBackoffAndJitter) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
