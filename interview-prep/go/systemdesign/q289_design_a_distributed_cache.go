// Question #289: Design a Distributed Cache
// Category: System Design | Difficulty: Hard
// Concepts: cache, eviction, sharding, failover
// Description: Design a Memcached/Redis-like cache with eviction, sharding, and failover.
package systemdesign

import "sync"

// Design a Distributed Cache
// Implements a system design component for question #289.
type DesignADistributedCache struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignADistributedCache creates a new system component.
func NewDesignADistributedCache() *DesignADistributedCache {
        return &DesignADistributedCache{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignADistributedCache) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignADistributedCache) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignADistributedCache) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
