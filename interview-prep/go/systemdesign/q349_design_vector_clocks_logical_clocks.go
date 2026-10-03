// Question #349: Design Vector Clocks / Logical Clocks
// Category: System Design | Difficulty: Hard
// Concepts: vector clock, logical clock, concurrency, causality
// Description: Use vector clocks to detect concurrent updates in a distributed store.
package systemdesign

import "sync"

// Design Vector Clocks / Logical Clocks
// Implements a system design component for question #349.
type Q349_DesignVectorClocksLogicalClocks struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ349_DesignVectorClocksLogicalClocks creates a new system component.
func NewQ349_DesignVectorClocksLogicalClocks() *Q349_DesignVectorClocksLogicalClocks {
        return &Q349_DesignVectorClocksLogicalClocks{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q349_DesignVectorClocksLogicalClocks) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q349_DesignVectorClocksLogicalClocks) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q349_DesignVectorClocksLogicalClocks) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
