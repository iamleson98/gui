// Question #291: Design a Rate Limiter (Sliding Window)
// Category: System Design | Difficulty: Hard
// Concepts: rate limiter, sliding window, sorted set, accuracy
// Description: Implement a sliding-window rate limiter using sorted sets or a rolling counter sketch.
package systemdesign

import "sync"

// Design a Rate Limiter (Sliding Window)
// Implements a system design component for question #291.
type DesignARateLimiterSlidingWindow struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignARateLimiterSlidingWindow creates a new system component.
func NewDesignARateLimiterSlidingWindow() *DesignARateLimiterSlidingWindow {
        return &DesignARateLimiterSlidingWindow{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignARateLimiterSlidingWindow) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignARateLimiterSlidingWindow) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignARateLimiterSlidingWindow) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
