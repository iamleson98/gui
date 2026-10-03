// Question #313: Design a Distributed Counter (Likes)
// Category: System Design | Difficulty: Hard
// Concepts: counter, CRDT, sharded, eventual
// Description: Design an eventually consistent like counter using CRDTs and sharded counts.
package systemdesign

import "sync"

// Design a Distributed Counter (Likes)
// Implements a system design component for question #313.
type Q313_DesignADistributedCounterLikes struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ313_DesignADistributedCounterLikes creates a new system component.
func NewQ313_DesignADistributedCounterLikes() *Q313_DesignADistributedCounterLikes {
        return &Q313_DesignADistributedCounterLikes{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q313_DesignADistributedCounterLikes) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q313_DesignADistributedCounterLikes) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q313_DesignADistributedCounterLikes) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
