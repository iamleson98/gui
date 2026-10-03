// Question #358: Design an A/B Testing Platform
// Category: System Design | Difficulty: Hard
// Concepts: A/B testing, bucketing, metrics, significance
// Description: Design an experimentation platform with bucketing, metrics, and statistical guardrails.
package systemdesign

import "sync"

// Design an A/B Testing Platform
// Implements a system design component for question #358.
type Q358_DesignAnABTestingPlatform struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ358_DesignAnABTestingPlatform creates a new system component.
func NewQ358_DesignAnABTestingPlatform() *Q358_DesignAnABTestingPlatform {
        return &Q358_DesignAnABTestingPlatform{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q358_DesignAnABTestingPlatform) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q358_DesignAnABTestingPlatform) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q358_DesignAnABTestingPlatform) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
