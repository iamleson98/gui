// Question #360: Design an ML Feature Store
// Category: System Design | Difficulty: Hard
// Concepts: feature store, online/offline, consistency, serving
// Description: Design a feature store serving consistent features online and offline.
package systemdesign

import "sync"

// Design an ML Feature Store
// Implements a system design component for question #360.
type Q360_DesignAnMlFeatureStore struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ360_DesignAnMlFeatureStore creates a new system component.
func NewQ360_DesignAnMlFeatureStore() *Q360_DesignAnMlFeatureStore {
        return &Q360_DesignAnMlFeatureStore{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q360_DesignAnMlFeatureStore) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q360_DesignAnMlFeatureStore) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q360_DesignAnMlFeatureStore) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
