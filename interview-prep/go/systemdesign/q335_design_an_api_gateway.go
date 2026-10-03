// Question #335: Design an API Gateway
// Category: System Design | Difficulty: Hard
// Concepts: API gateway, auth, rate limit, routing
// Description: Design an API gateway with auth, rate limiting, routing, and observability.
package systemdesign

import "sync"

// Design an API Gateway
// Implements a system design component for question #335.
type DesignAnApiGateway struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAnApiGateway creates a new system component.
func NewDesignAnApiGateway() *DesignAnApiGateway {
        return &DesignAnApiGateway{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAnApiGateway) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAnApiGateway) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAnApiGateway) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
