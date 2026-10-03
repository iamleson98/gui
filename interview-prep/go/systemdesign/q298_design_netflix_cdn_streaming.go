// Question #298: Design Netflix / CDN Streaming
// Category: System Design | Difficulty: Hard
// Concepts: streaming CDN, origin shield, cache tier, ABR
// Description: Design a streaming CDN with origin shields, caching tiers, and ABR playback.
package systemdesign

import "sync"

// Design Netflix / CDN Streaming
// Implements a system design component for question #298.
type Q298_DesignNetflixCdnStreaming struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ298_DesignNetflixCdnStreaming creates a new system component.
func NewQ298_DesignNetflixCdnStreaming() *Q298_DesignNetflixCdnStreaming {
        return &Q298_DesignNetflixCdnStreaming{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q298_DesignNetflixCdnStreaming) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q298_DesignNetflixCdnStreaming) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q298_DesignNetflixCdnStreaming) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
