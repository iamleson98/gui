// Question #288: Design a Key-Value Store
// Category: System Design | Difficulty: Hard
// Concepts: key-value store, consistent hashing, replication, quorum
// Description: Design a distributed key-value store with consistent hashing, replication, and tunable consistency.
package systemdesign

import "sync"

// Design a Key-Value Store
// Implements a system design component for question #288.
type DesignAKeyValueStore struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAKeyValueStore creates a new system component.
func NewDesignAKeyValueStore() *DesignAKeyValueStore {
        return &DesignAKeyValueStore{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAKeyValueStore) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAKeyValueStore) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAKeyValueStore) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
