// Question #336: Design a BFF (Backend for Frontend)
// Category: System Design | Difficulty: Hard
// Concepts: BFF, aggregation, client-specific, edge
// Description: Design a backend-for-frontend layer that aggregates services for a specific client.
package systemdesign

import "sync"

// Design a BFF (Backend for Frontend)
// Implements a system design component for question #336.
type DesignABffBackendForFrontend struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignABffBackendForFrontend creates a new system component.
func NewDesignABffBackendForFrontend() *DesignABffBackendForFrontend {
        return &DesignABffBackendForFrontend{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignABffBackendForFrontend) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignABffBackendForFrontend) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignABffBackendForFrontend) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
