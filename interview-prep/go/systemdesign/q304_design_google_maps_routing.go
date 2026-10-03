// Question #304: Design Google Maps / Routing
// Category: System Design | Difficulty: Hard
// Concepts: routing, A*, contraction hierarchies, road graph
// Description:  Design a routing service using contraction hierarchies and A* on a road graph.
package systemdesign

import "sync"

// Design Google Maps / Routing
// Implements a system design component for question #304.
type DesignGoogleMapsRouting struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignGoogleMapsRouting creates a new system component.
func NewDesignGoogleMapsRouting() *DesignGoogleMapsRouting {
        return &DesignGoogleMapsRouting{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignGoogleMapsRouting) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignGoogleMapsRouting) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignGoogleMapsRouting) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
