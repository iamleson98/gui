// Question #329: Design a CDN
// Category: System Design | Difficulty: Hard
// Concepts: CDN, edge cache, origin pull, invalidation
// Description: Design a CDN with edge caches, origin pull, and cache invalidation strategies.
package systemdesign

import "sync"

// Design a CDN
// Implements a system design component for question #329.
type Q329_DesignACdn struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ329_DesignACdn creates a new system component.
func NewQ329_DesignACdn() *Q329_DesignACdn {
        return &Q329_DesignACdn{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q329_DesignACdn) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q329_DesignACdn) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q329_DesignACdn) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
