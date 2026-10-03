// Question #351: Design a Quorum System
// Category: System Design | Difficulty: Hard
// Concepts: quorum, R/W, consistency, latency
// Description: Design tunable R/W quorums trading consistency for latency and availability.
package systemdesign

import "sync"

// Design a Quorum System
// Implements a system design component for question #351.
type DesignAQuorumSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAQuorumSystem creates a new system component.
func NewDesignAQuorumSystem() *DesignAQuorumSystem {
        return &DesignAQuorumSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAQuorumSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAQuorumSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAQuorumSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
