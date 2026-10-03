// Question #338: Design a Bulkhead Pattern
// Category: System Design | Difficulty: Hard
// Concepts: bulkhead, isolation, concurrency limit, failure domain
// Description: Isolate failure domains with bulkheads limiting concurrency per dependency.
package systemdesign

import "sync"

// Design a Bulkhead Pattern
// Implements a system design component for question #338.
type Q338_DesignABulkheadPattern struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ338_DesignABulkheadPattern creates a new system component.
func NewQ338_DesignABulkheadPattern() *Q338_DesignABulkheadPattern {
        return &Q338_DesignABulkheadPattern{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q338_DesignABulkheadPattern) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q338_DesignABulkheadPattern) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q338_DesignABulkheadPattern) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
