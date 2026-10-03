// Question #292: Design Twitter / News Feed
// Category: System Design | Difficulty: Hard
// Concepts: news feed, fan-out, timeline, ranking
// Description: Design a timeline service deciding between fan-out on write and fan-out on read.
package systemdesign

import "sync"

// Design Twitter / News Feed
// Implements a system design component for question #292.
type DesignTwitterNewsFeed struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignTwitterNewsFeed creates a new system component.
func NewDesignTwitterNewsFeed() *DesignTwitterNewsFeed {
        return &DesignTwitterNewsFeed{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignTwitterNewsFeed) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignTwitterNewsFeed) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignTwitterNewsFeed) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
