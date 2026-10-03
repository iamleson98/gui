// Question #312: Design Idempotent Payment Processing
// Category: System Design | Difficulty: Hard
// Concepts: idempotency, payment, dedup, ledger
// Description: Ensure payment APIs are idempotent using idempotency keys and a dedup store.
package systemdesign

import "sync"

// Design Idempotent Payment Processing
// Implements a system design component for question #312.
type Q312_DesignIdempotentPaymentProcessing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ312_DesignIdempotentPaymentProcessing creates a new system component.
func NewQ312_DesignIdempotentPaymentProcessing() *Q312_DesignIdempotentPaymentProcessing {
        return &Q312_DesignIdempotentPaymentProcessing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q312_DesignIdempotentPaymentProcessing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q312_DesignIdempotentPaymentProcessing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q312_DesignIdempotentPaymentProcessing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
