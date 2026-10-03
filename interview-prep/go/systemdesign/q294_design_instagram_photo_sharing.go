// Question #294: Design Instagram / Photo Sharing
// Category: System Design | Difficulty: Hard
// Concepts: photo sharing, object storage, CDN, metadata
// Description: Design a photo-sharing service with object storage, CDN, and metadata sharding.
package systemdesign

import "sync"

// Design Instagram / Photo Sharing
// Implements a system design component for question #294.
type Q294_DesignInstagramPhotoSharing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ294_DesignInstagramPhotoSharing creates a new system component.
func NewQ294_DesignInstagramPhotoSharing() *Q294_DesignInstagramPhotoSharing {
        return &Q294_DesignInstagramPhotoSharing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q294_DesignInstagramPhotoSharing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q294_DesignInstagramPhotoSharing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q294_DesignInstagramPhotoSharing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
