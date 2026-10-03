// Question #308: Design an Event Ticketing System
// Category: System Design | Difficulty: Hard
// Concepts: flash sale, virtual queue, inventory, contention
// Description: Design high-contention flash-sale ticketing with virtual queues and inventory sharding.
package systemdesign

import "sync"

// Design an Event Ticketing System
// Implements a system design component for question #308.
type Q308_DesignAnEventTicketingSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ308_DesignAnEventTicketingSystem creates a new system component.
func NewQ308_DesignAnEventTicketingSystem() *Q308_DesignAnEventTicketingSystem {
        return &Q308_DesignAnEventTicketingSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q308_DesignAnEventTicketingSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q308_DesignAnEventTicketingSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q308_DesignAnEventTicketingSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
