// Question #355: Design a Gossip Protocol
// Category: System Design | Difficulty: Hard
// Concepts: gossip, membership, epidemic, bounded
// Description: Propagate membership and state updates across nodes with a bounded gossip round.
package systemdesign

import "sync"

// Design a Gossip Protocol
// Implements a system design component for question #355.
type Q355_DesignAGossipProtocol struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ355_DesignAGossipProtocol creates a new system component.
func NewQ355_DesignAGossipProtocol() *Q355_DesignAGossipProtocol {
        return &Q355_DesignAGossipProtocol{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q355_DesignAGossipProtocol) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q355_DesignAGossipProtocol) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q355_DesignAGossipProtocol) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
