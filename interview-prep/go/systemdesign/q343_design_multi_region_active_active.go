// Question #343: Design Multi-Region Active-Active
// Category: System Design | Difficulty: Hard
// Concepts: active-active, multi-region, conflict, routing
// Description: Design an active-active multi-region system handling conflict resolution and routing.
package systemdesign

import "sync"

// Design Multi-Region Active-Active
// Implements a system design component for question #343.
type Q343_DesignMultiRegionActiveActive struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ343_DesignMultiRegionActiveActive creates a new system component.
func NewQ343_DesignMultiRegionActiveActive() *Q343_DesignMultiRegionActiveActive {
        return &Q343_DesignMultiRegionActiveActive{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q343_DesignMultiRegionActiveActive) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q343_DesignMultiRegionActiveActive) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q343_DesignMultiRegionActiveActive) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
