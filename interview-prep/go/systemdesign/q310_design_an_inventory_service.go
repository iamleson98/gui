// Question #310: Design an Inventory Service
// Category: System Design | Difficulty: Hard
// Concepts: inventory, reservation, strong consistency, SKU
// Description: Design a per-SKU inventory service with strong consistency and reservation semantics.
package systemdesign

import "sync"

// Design an Inventory Service
// Implements a system design component for question #310.
type Q310_DesignAnInventoryService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ310_DesignAnInventoryService creates a new system component.
func NewQ310_DesignAnInventoryService() *Q310_DesignAnInventoryService {
        return &Q310_DesignAnInventoryService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q310_DesignAnInventoryService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q310_DesignAnInventoryService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q310_DesignAnInventoryService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
