// Question #337: Design a Circuit Breaker
// Category: System Design | Difficulty: Hard
// Concepts: circuit breaker, half-open, thresholds, resilience
// Description: Implement a circuit breaker with half-open probing and configurable thresholds.
package systemdesign

import "sync"

// Design a Circuit Breaker
// Implements a system design component for question #337.
type Q337_DesignACircuitBreaker struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ337_DesignACircuitBreaker creates a new system component.
func NewQ337_DesignACircuitBreaker() *Q337_DesignACircuitBreaker {
        return &Q337_DesignACircuitBreaker{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q337_DesignACircuitBreaker) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q337_DesignACircuitBreaker) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q337_DesignACircuitBreaker) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
