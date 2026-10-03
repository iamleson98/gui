// Question #311: Design a Payment System
// Category: System Design | Difficulty: Hard
// Concepts: payments, ledger, idempotency, reconciliation
// Description: Design a payment processing system with idempotency, ledgering, and reconciliation.
package systemdesign

import "sync"

// Design a Payment System
// Implements a system design component for question #311.
type Q311_DesignAPaymentSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ311_DesignAPaymentSystem creates a new system component.
func NewQ311_DesignAPaymentSystem() *Q311_DesignAPaymentSystem {
        return &Q311_DesignAPaymentSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q311_DesignAPaymentSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q311_DesignAPaymentSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q311_DesignAPaymentSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
