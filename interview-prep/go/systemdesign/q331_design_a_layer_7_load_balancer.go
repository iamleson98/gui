// Question #331: Design a Layer-7 Load Balancer
// Category: System Design | Difficulty: Hard
// Concepts: L7 LB, content routing, TLS termination, retries
// Description: Design an L7 load balancer with content-based routing, TLS termination, and retries.
package systemdesign

import "sync"

// Design a Layer-7 Load Balancer
// Implements a system design component for question #331.
type DesignALayer7LoadBalancer struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignALayer7LoadBalancer creates a new system component.
func NewDesignALayer7LoadBalancer() *DesignALayer7LoadBalancer {
        return &DesignALayer7LoadBalancer{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignALayer7LoadBalancer) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignALayer7LoadBalancer) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignALayer7LoadBalancer) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
