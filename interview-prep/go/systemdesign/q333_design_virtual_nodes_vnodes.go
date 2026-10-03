// Question #333: Design Virtual Nodes (Vnodes)
// Category: System Design | Difficulty: Hard
// Concepts: vnodes, load balancing, hotspots, ring
// Description: Add virtual nodes to consistent hashing to smooth load and reduce hotspots.
package systemdesign

import "sync"

// Design Virtual Nodes (Vnodes)
// Implements a system design component for question #333.
type DesignVirtualNodesVnodes struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignVirtualNodesVnodes creates a new system component.
func NewDesignVirtualNodesVnodes() *DesignVirtualNodesVnodes {
        return &DesignVirtualNodesVnodes{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignVirtualNodesVnodes) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignVirtualNodesVnodes) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignVirtualNodesVnodes) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
