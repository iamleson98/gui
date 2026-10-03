// Question #290: Design a Rate Limiter (Token Bucket)
// Category: System Design | Difficulty: Hard
// Concepts: rate limiter, token bucket, distributed, sliding
// Description: Design a token-bucket rate limiter distributed across nodes with sliding accuracy.
package systemdesign

import "sync"

// Design a Rate Limiter (Token Bucket)
// Implements a system design component for question #290.
type Q290_DesignARateLimiterTokenBucket struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ290_DesignARateLimiterTokenBucket creates a new system component.
func NewQ290_DesignARateLimiterTokenBucket() *Q290_DesignARateLimiterTokenBucket {
        return &Q290_DesignARateLimiterTokenBucket{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q290_DesignARateLimiterTokenBucket) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q290_DesignARateLimiterTokenBucket) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q290_DesignARateLimiterTokenBucket) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
