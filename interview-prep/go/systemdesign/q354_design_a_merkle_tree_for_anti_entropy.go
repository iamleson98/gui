// Question #354: Design a Merkle Tree for Anti-Entropy
// Category: System Design | Difficulty: Hard
// Concepts: Merkle tree, anti-entropy, replica diff, repair
// Description: Compare replicas with Merkle trees to localize and repair differences.
package systemdesign

import "sync"

// Design a Merkle Tree for Anti-Entropy
// Implements a system design component for question #354.
type DesignAMerkleTreeForAntiEntropy struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAMerkleTreeForAntiEntropy creates a new system component.
func NewDesignAMerkleTreeForAntiEntropy() *DesignAMerkleTreeForAntiEntropy {
        return &DesignAMerkleTreeForAntiEntropy{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAMerkleTreeForAntiEntropy) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAMerkleTreeForAntiEntropy) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAMerkleTreeForAntiEntropy) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
