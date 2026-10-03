// Question #327: Design an Email Service
// Category: System Design | Difficulty: Hard
// Concepts: email, provider failover, bounce, throttling
// Description: Design a transactional email service with provider failover and bounce handling.
package systemdesign

import "sync"

// Design an Email Service
// Implements a system design component for question #327.
type Q327_DesignAnEmailService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ327_DesignAnEmailService creates a new system component.
func NewQ327_DesignAnEmailService() *Q327_DesignAnEmailService {
        return &Q327_DesignAnEmailService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q327_DesignAnEmailService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q327_DesignAnEmailService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q327_DesignAnEmailService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
