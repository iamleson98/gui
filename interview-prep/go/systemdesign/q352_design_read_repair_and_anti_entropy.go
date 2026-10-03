// Question #352: Design Read-Repair and Anti-Entropy
// Category: System Design | Difficulty: Hard
// Concepts: read repair, anti-entropy, replica, consistency
// Description: Repair divergent replicas via read-repair and background anti-entropy.
package systemdesign

import "sync"

// Design Read-Repair and Anti-Entropy
// Implements a system design component for question #352.
type DesignReadRepairAndAntiEntropy struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignReadRepairAndAntiEntropy creates a new system component.
func NewDesignReadRepairAndAntiEntropy() *DesignReadRepairAndAntiEntropy {
        return &DesignReadRepairAndAntiEntropy{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignReadRepairAndAntiEntropy) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignReadRepairAndAntiEntropy) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignReadRepairAndAntiEntropy) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
