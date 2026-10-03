// Question #352: Design Read-Repair and Anti-Entropy
// Category: System Design | Difficulty: Hard
// Concepts: read repair, anti-entropy, replica, consistency
// Description: Repair divergent replicas via read-repair and background anti-entropy.
package systemdesign

import "sync"

// Design Read-Repair and Anti-Entropy
// Implements a system design component for question #352.
type Q352_DesignReadRepairAndAntiEntropy struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ352_DesignReadRepairAndAntiEntropy creates a new system component.
func NewQ352_DesignReadRepairAndAntiEntropy() *Q352_DesignReadRepairAndAntiEntropy {
        return &Q352_DesignReadRepairAndAntiEntropy{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q352_DesignReadRepairAndAntiEntropy) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q352_DesignReadRepairAndAntiEntropy) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q352_DesignReadRepairAndAntiEntropy) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
