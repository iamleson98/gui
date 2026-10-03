// Question #308: Design an Event Ticketing System
// Category: System Design | Difficulty: Hard
// Concepts: flash sale, virtual queue, inventory, contention
// Description: Design high-contention flash-sale ticketing with virtual queues and inventory sharding.
package systemdesign

import "sync"

// Design an Event Ticketing System
// Implements a system design component for question #308.
type DesignAnEventTicketingSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAnEventTicketingSystem creates a new system component.
func NewDesignAnEventTicketingSystem() *DesignAnEventTicketingSystem {
        return &DesignAnEventTicketingSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAnEventTicketingSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAnEventTicketingSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAnEventTicketingSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
