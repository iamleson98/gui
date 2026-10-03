// Question #328: Design a DNS System
// Category: System Design | Difficulty: Hard
// Concepts: DNS, hierarchy, TTL, negative caching
// Description: Design a hierarchical, cached DNS resolver with TTLs and negative caching.
package systemdesign

import "sync"

// Design a DNS System
// Implements a system design component for question #328.
type Q328_DesignADnsSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ328_DesignADnsSystem creates a new system component.
func NewQ328_DesignADnsSystem() *Q328_DesignADnsSystem {
        return &Q328_DesignADnsSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q328_DesignADnsSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q328_DesignADnsSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q328_DesignADnsSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
