// Question #344: Design Multi-Region Active-Passive
// Category: System Design | Difficulty: Hard
// Concepts: active-passive, failover, replication, RTO
// Description: Design an active-passive multi-region system with failover and data replication.
package systemdesign

import "sync"

// Design Multi-Region Active-Passive
// Implements a system design component for question #344.
type Q344_DesignMultiRegionActivePassive struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ344_DesignMultiRegionActivePassive creates a new system component.
func NewQ344_DesignMultiRegionActivePassive() *Q344_DesignMultiRegionActivePassive {
        return &Q344_DesignMultiRegionActivePassive{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q344_DesignMultiRegionActivePassive) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q344_DesignMultiRegionActivePassive) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q344_DesignMultiRegionActivePassive) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
