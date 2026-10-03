// Question #348: Design a Consensus-Based Config Store
// Category: System Design | Difficulty: Hard
// Concepts: config store, Raft, watchers, linearizable
// Description: Design an etcd/ZooKeeper-like config store using Raft and watchers.
package systemdesign

import "sync"

// Design a Consensus-Based Config Store
// Implements a system design component for question #348.
type Q348_DesignAConsensusBasedConfigStore struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ348_DesignAConsensusBasedConfigStore creates a new system component.
func NewQ348_DesignAConsensusBasedConfigStore() *Q348_DesignAConsensusBasedConfigStore {
        return &Q348_DesignAConsensusBasedConfigStore{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q348_DesignAConsensusBasedConfigStore) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q348_DesignAConsensusBasedConfigStore) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q348_DesignAConsensusBasedConfigStore) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
