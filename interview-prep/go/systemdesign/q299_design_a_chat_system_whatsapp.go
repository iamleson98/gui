// Question #299: Design a Chat System (WhatsApp)
// Category: System Design | Difficulty: Hard
// Concepts: chat, presence, delivery receipts, offline sync
// Description: Design a real-time chat service with presence, delivery receipts, and offline sync.
package systemdesign

import "sync"

// Design a Chat System (WhatsApp)
// Implements a system design component for question #299.
type Q299_DesignAChatSystemWhatsapp struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ299_DesignAChatSystemWhatsapp creates a new system component.
func NewQ299_DesignAChatSystemWhatsapp() *Q299_DesignAChatSystemWhatsapp {
        return &Q299_DesignAChatSystemWhatsapp{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q299_DesignAChatSystemWhatsapp) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q299_DesignAChatSystemWhatsapp) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q299_DesignAChatSystemWhatsapp) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
