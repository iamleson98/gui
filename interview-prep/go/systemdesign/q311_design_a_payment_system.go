// Question #311: Design a Payment System
// Category: System Design | Difficulty: Hard
// Concepts: payments, ledger, idempotency, reconciliation
// Description: Design a payment processing system with idempotency, ledgering, and reconciliation.
package systemdesign

import "sync"

// Design a Payment System
// Implements a system design component for question #311.
type DesignAPaymentSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAPaymentSystem creates a new system component.
func NewDesignAPaymentSystem() *DesignAPaymentSystem {
        return &DesignAPaymentSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAPaymentSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAPaymentSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAPaymentSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
