// Question #330: Design a Layer-4 Load Balancer
// Category: System Design | Difficulty: Hard
// Concepts: L4 LB, connection tracking, consistent hashing, NAT/DSR
// Description: Design an L4 load balancer using connection tracking and consistent hashing.
package systemdesign

import "sync"

// Design a Layer-4 Load Balancer
// Implements a system design component for question #330.
type DesignALayer4LoadBalancer struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignALayer4LoadBalancer creates a new system component.
func NewDesignALayer4LoadBalancer() *DesignALayer4LoadBalancer {
        return &DesignALayer4LoadBalancer{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignALayer4LoadBalancer) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignALayer4LoadBalancer) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignALayer4LoadBalancer) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
