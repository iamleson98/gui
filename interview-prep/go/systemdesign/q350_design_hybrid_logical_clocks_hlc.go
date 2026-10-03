// Question #350: Design Hybrid Logical Clocks (HLC)
// Category: System Design | Difficulty: Hard
// Concepts: HLC, physical, logical, drift
// Description: Combine physical and logical time into HLCs for bounded drift ordering.
package systemdesign

import "sync"

// Design Hybrid Logical Clocks (HLC)
// Implements a system design component for question #350.
type Q350_DesignHybridLogicalClocksHlc struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ350_DesignHybridLogicalClocksHlc creates a new system component.
func NewQ350_DesignHybridLogicalClocksHlc() *Q350_DesignHybridLogicalClocksHlc {
        return &Q350_DesignHybridLogicalClocksHlc{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q350_DesignHybridLogicalClocksHlc) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q350_DesignHybridLogicalClocksHlc) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q350_DesignHybridLogicalClocksHlc) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
