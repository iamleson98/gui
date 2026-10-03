// Question #336: Design a BFF (Backend for Frontend)
// Category: System Design | Difficulty: Hard
// Concepts: BFF, aggregation, client-specific, edge
// Description: Design a backend-for-frontend layer that aggregates services for a specific client.
package systemdesign

import "sync"

// Design a BFF (Backend for Frontend)
// Implements a system design component for question #336.
type Q336_DesignABffBackendForFrontend struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ336_DesignABffBackendForFrontend creates a new system component.
func NewQ336_DesignABffBackendForFrontend() *Q336_DesignABffBackendForFrontend {
        return &Q336_DesignABffBackendForFrontend{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q336_DesignABffBackendForFrontend) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q336_DesignABffBackendForFrontend) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q336_DesignABffBackendForFrontend) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
