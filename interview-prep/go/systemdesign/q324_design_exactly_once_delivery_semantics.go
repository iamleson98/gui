// Question #324: Design Exactly-Once Delivery Semantics
// Category: System Design | Difficulty: Hard
// Concepts: exactly-once, idempotent producer, transactions, EOS
// Description: Achieve exactly-once delivery using idempotent producers and transactional consumption.
package systemdesign

import "sync"

// Design Exactly-Once Delivery Semantics
// Implements a system design component for question #324.
type Q324_DesignExactlyOnceDeliverySemantics struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ324_DesignExactlyOnceDeliverySemantics creates a new system component.
func NewQ324_DesignExactlyOnceDeliverySemantics() *Q324_DesignExactlyOnceDeliverySemantics {
        return &Q324_DesignExactlyOnceDeliverySemantics{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q324_DesignExactlyOnceDeliverySemantics) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q324_DesignExactlyOnceDeliverySemantics) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q324_DesignExactlyOnceDeliverySemantics) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
