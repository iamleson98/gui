// Question #315: Design a Comment System
// Category: System Design | Difficulty: Hard
// Concepts: comments, threading, materialized path, pagination
// Description: Design a threaded comment system with materialized paths and pagination.
package systemdesign

import "sync"

// Design a Comment System
// Implements a system design component for question #315.
type DesignACommentSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignACommentSystem creates a new system component.
func NewDesignACommentSystem() *DesignACommentSystem {
        return &DesignACommentSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignACommentSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignACommentSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignACommentSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
