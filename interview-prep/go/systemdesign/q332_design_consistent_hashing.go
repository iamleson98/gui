// Question #332: Design Consistent Hashing
// Category: System Design | Difficulty: Hard
// Concepts: consistent hashing, virtual nodes, ring, reshuffle
// Description: Implement consistent hashing with virtual nodes for balanced, low-reshuffle distribution.
package systemdesign

import "sync"

// Design Consistent Hashing
// Implements a system design component for question #332.
type Q332_DesignConsistentHashing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ332_DesignConsistentHashing creates a new system component.
func NewQ332_DesignConsistentHashing() *Q332_DesignConsistentHashing {
        return &Q332_DesignConsistentHashing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q332_DesignConsistentHashing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q332_DesignConsistentHashing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q332_DesignConsistentHashing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
