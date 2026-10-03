// Question #347: Design a Distributed Lock Service
// Category: System Design | Difficulty: Hard
// Concepts: distributed lock, fencing token, lease, renewal
// Description: Design a distributed lock with fencing tokens and lease renewal.
package systemdesign

import "sync"

// Design a Distributed Lock Service
// Implements a system design component for question #347.
type Q347_DesignADistributedLockService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ347_DesignADistributedLockService creates a new system component.
func NewQ347_DesignADistributedLockService() *Q347_DesignADistributedLockService {
        return &Q347_DesignADistributedLockService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q347_DesignADistributedLockService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q347_DesignADistributedLockService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q347_DesignADistributedLockService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
