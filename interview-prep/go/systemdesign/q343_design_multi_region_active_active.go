// Question #343: Design Multi-Region Active-Active
// Category: System Design | Difficulty: Hard
// Concepts: active-active, multi-region, conflict, routing
// Description: Design an active-active multi-region system handling conflict resolution and routing.
package systemdesign

import "sync"

// Design Multi-Region Active-Active
// Implements a system design component for question #343.
type DesignMultiRegionActiveActive struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignMultiRegionActiveActive creates a new system component.
func NewDesignMultiRegionActiveActive() *DesignMultiRegionActiveActive {
        return &DesignMultiRegionActiveActive{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignMultiRegionActiveActive) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignMultiRegionActiveActive) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignMultiRegionActiveActive) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
