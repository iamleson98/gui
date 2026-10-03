// Question #305: Design a Ride-Sharing System (Uber)
// Category: System Design | Difficulty: Hard
// Concepts: ride sharing, geospatial, dispatch, surge
// Description: Design ride matching with geospatial indexing, surge pricing, and dispatch.
package systemdesign

import "sync"

// Design a Ride-Sharing System (Uber)
// Implements a system design component for question #305.
type DesignARideSharingSystemUber struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignARideSharingSystemUber creates a new system component.
func NewDesignARideSharingSystemUber() *DesignARideSharingSystemUber {
        return &DesignARideSharingSystemUber{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignARideSharingSystemUber) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignARideSharingSystemUber) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignARideSharingSystemUber) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
