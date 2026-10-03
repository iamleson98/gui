// Question #293: Design Twitter Timeline Fan-Out
// Category: System Design | Difficulty: Hard
// Concepts: fan-out, push, pull, celebrity
// Description: Compare push-on-write vs pull-on-read fan-out for celebrity and normal users.
package systemdesign

import "sync"

// Design Twitter Timeline Fan-Out
// Implements a system design component for question #293.
type Q293_DesignTwitterTimelineFanOut struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ293_DesignTwitterTimelineFanOut creates a new system component.
func NewQ293_DesignTwitterTimelineFanOut() *Q293_DesignTwitterTimelineFanOut {
        return &Q293_DesignTwitterTimelineFanOut{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q293_DesignTwitterTimelineFanOut) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q293_DesignTwitterTimelineFanOut) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q293_DesignTwitterTimelineFanOut) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
