// Question #350: Design Hybrid Logical Clocks (HLC)
// Category: System Design | Difficulty: Hard
// Concepts: HLC, physical, logical, drift
// Description: Combine physical and logical time into HLCs for bounded drift ordering.
package systemdesign

import "sync"

// Design Hybrid Logical Clocks (HLC)
// Implements a system design component for question #350.
type DesignHybridLogicalClocksHlc struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignHybridLogicalClocksHlc creates a new system component.
func NewDesignHybridLogicalClocksHlc() *DesignHybridLogicalClocksHlc {
        return &DesignHybridLogicalClocksHlc{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignHybridLogicalClocksHlc) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignHybridLogicalClocksHlc) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignHybridLogicalClocksHlc) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
