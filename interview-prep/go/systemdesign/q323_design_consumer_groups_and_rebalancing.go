// Question #323: Design Consumer Groups and Rebalancing
// Category: System Design | Difficulty: Hard
// Concepts: consumer group, rebalancing, coordinator, session
// Description: Design consumer group coordination with rebalancing strategies and session timeouts.
package systemdesign

import "sync"

// Design Consumer Groups and Rebalancing
// Implements a system design component for question #323.
type Q323_DesignConsumerGroupsAndRebalancing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ323_DesignConsumerGroupsAndRebalancing creates a new system component.
func NewQ323_DesignConsumerGroupsAndRebalancing() *Q323_DesignConsumerGroupsAndRebalancing {
        return &Q323_DesignConsumerGroupsAndRebalancing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q323_DesignConsumerGroupsAndRebalancing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q323_DesignConsumerGroupsAndRebalancing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q323_DesignConsumerGroupsAndRebalancing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
