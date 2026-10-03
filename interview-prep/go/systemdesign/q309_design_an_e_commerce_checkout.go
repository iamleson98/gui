// Question #309: Design an E-Commerce Checkout
// Category: System Design | Difficulty: Hard
// Concepts: checkout, cart, pricing, payment
// Description: Design a checkout pipeline with cart, pricing, inventory, and payment orchestration.
package systemdesign

import "sync"

// Design an E-Commerce Checkout
// Implements a system design component for question #309.
type DesignAnECommerceCheckout struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAnECommerceCheckout creates a new system component.
func NewDesignAnECommerceCheckout() *DesignAnECommerceCheckout {
        return &DesignAnECommerceCheckout{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAnECommerceCheckout) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAnECommerceCheckout) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAnECommerceCheckout) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
