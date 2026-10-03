// Question #340: Design a Feature Flag Service
// Category: System Design | Difficulty: Hard
// Concepts: feature flags, targeting, live config, rollouts
// Description: Design a feature flag service with targeting rules and live config updates.
package systemdesign

import "sync"

// Design a Feature Flag Service
// Implements a system design component for question #340.
type DesignAFeatureFlagService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAFeatureFlagService creates a new system component.
func NewDesignAFeatureFlagService() *DesignAFeatureFlagService {
        return &DesignAFeatureFlagService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAFeatureFlagService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAFeatureFlagService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAFeatureFlagService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
