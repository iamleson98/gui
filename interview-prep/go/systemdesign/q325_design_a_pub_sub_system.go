// Question #325: Design a Pub/Sub System
// Category: System Design | Difficulty: Hard
// Concepts: pub/sub, topics, durable, backpressure
// Description: Design a topic-based pub/sub with durable subscriptions and backpressure.
package systemdesign

import "sync"

// Design a Pub/Sub System
// Implements a system design component for question #325.
type DesignAPubSubSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAPubSubSystem creates a new system component.
func NewDesignAPubSubSystem() *DesignAPubSubSystem {
        return &DesignAPubSubSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAPubSubSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAPubSubSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAPubSubSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
