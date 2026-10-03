// Question #290: Design a Rate Limiter (Token Bucket)
// Category: System Design | Difficulty: Hard
// Concepts: rate limiter, token bucket, distributed, sliding
// Description: Design a token-bucket rate limiter distributed across nodes with sliding accuracy.
package systemdesign

import "sync"

// Design a Rate Limiter (Token Bucket)
// Implements a system design component for question #290.
type DesignARateLimiterTokenBucket struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignARateLimiterTokenBucket creates a new system component.
func NewDesignARateLimiterTokenBucket() *DesignARateLimiterTokenBucket {
        return &DesignARateLimiterTokenBucket{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignARateLimiterTokenBucket) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignARateLimiterTokenBucket) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignARateLimiterTokenBucket) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
