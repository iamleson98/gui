// Question #304: Design Google Maps / Routing
// Category: System Design | Difficulty: Hard
// Concepts: routing, A*, contraction hierarchies, road graph
// Description:  Design a routing service using contraction hierarchies and A* on a road graph.
package systemdesign

import "sync"

// Design Google Maps / Routing
// Implements a system design component for question #304.
type Q304_DesignGoogleMapsRouting struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ304_DesignGoogleMapsRouting creates a new system component.
func NewQ304_DesignGoogleMapsRouting() *Q304_DesignGoogleMapsRouting {
        return &Q304_DesignGoogleMapsRouting{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q304_DesignGoogleMapsRouting) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q304_DesignGoogleMapsRouting) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q304_DesignGoogleMapsRouting) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
