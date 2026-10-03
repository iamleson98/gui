// Question #312: Design Idempotent Payment Processing
// Category: System Design | Difficulty: Hard
// Concepts: idempotency, payment, dedup, ledger
// Description: Ensure payment APIs are idempotent using idempotency keys and a dedup store.
package systemdesign

import "sync"

// Design Idempotent Payment Processing
// Implements a system design component for question #312.
type DesignIdempotentPaymentProcessing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignIdempotentPaymentProcessing creates a new system component.
func NewDesignIdempotentPaymentProcessing() *DesignIdempotentPaymentProcessing {
        return &DesignIdempotentPaymentProcessing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignIdempotentPaymentProcessing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignIdempotentPaymentProcessing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignIdempotentPaymentProcessing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
