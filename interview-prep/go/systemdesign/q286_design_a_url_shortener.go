// Question #286: Design a URL Shortener
// Category: System Design | Difficulty: Hard
// Concepts: URL shortener, base62, sharding, cache
// Description: Design a service that maps long URLs to short codes with high read throughput and analytics.
package systemdesign

import "sync"

// Design a URL Shortener
// Implements a system design component for question #286.
type Q286_DesignAUrlShortener struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ286_DesignAUrlShortener creates a new system component.
func NewQ286_DesignAUrlShortener() *Q286_DesignAUrlShortener {
        return &Q286_DesignAUrlShortener{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q286_DesignAUrlShortener) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q286_DesignAUrlShortener) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q286_DesignAUrlShortener) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
