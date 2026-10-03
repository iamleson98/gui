// Question #354: Design a Merkle Tree for Anti-Entropy
// Category: System Design | Difficulty: Hard
// Concepts: Merkle tree, anti-entropy, replica diff, repair
// Description: Compare replicas with Merkle trees to localize and repair differences.
package systemdesign

import "sync"

// Design a Merkle Tree for Anti-Entropy
// Implements a system design component for question #354.
type Q354_DesignAMerkleTreeForAntiEntropy struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ354_DesignAMerkleTreeForAntiEntropy creates a new system component.
func NewQ354_DesignAMerkleTreeForAntiEntropy() *Q354_DesignAMerkleTreeForAntiEntropy {
        return &Q354_DesignAMerkleTreeForAntiEntropy{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q354_DesignAMerkleTreeForAntiEntropy) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q354_DesignAMerkleTreeForAntiEntropy) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q354_DesignAMerkleTreeForAntiEntropy) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
