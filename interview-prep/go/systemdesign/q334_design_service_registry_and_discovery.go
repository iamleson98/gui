// Question #334: Design Service Registry and Discovery
// Category: System Design | Difficulty: Hard
// Concepts: service discovery, registry, health checks, client-side
// Description: Design a service registry with health checks and client-side discovery.
package systemdesign

import "sync"

// Design Service Registry and Discovery
// Implements a system design component for question #334.
type Q334_DesignServiceRegistryAndDiscovery struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ334_DesignServiceRegistryAndDiscovery creates a new system component.
func NewQ334_DesignServiceRegistryAndDiscovery() *Q334_DesignServiceRegistryAndDiscovery {
        return &Q334_DesignServiceRegistryAndDiscovery{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q334_DesignServiceRegistryAndDiscovery) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q334_DesignServiceRegistryAndDiscovery) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q334_DesignServiceRegistryAndDiscovery) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
