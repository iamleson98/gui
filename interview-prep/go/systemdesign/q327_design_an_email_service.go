// Question #327: Design an Email Service
// Category: System Design | Difficulty: Hard
// Concepts: email, provider failover, bounce, throttling
// Description: Design a transactional email service with provider failover and bounce handling.
package systemdesign

import "sync"

// Design an Email Service
// Implements a system design component for question #327.
type DesignAnEmailService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAnEmailService creates a new system component.
func NewDesignAnEmailService() *DesignAnEmailService {
        return &DesignAnEmailService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAnEmailService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAnEmailService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAnEmailService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
