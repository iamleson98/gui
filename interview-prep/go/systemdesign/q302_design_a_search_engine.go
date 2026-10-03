// Question #302: Design a Search Engine
// Category: System Design | Difficulty: Hard
// Concepts: search, crawling, inverted index, ranking
// Description: Design a search engine with crawling, indexing, ranking, and query serving.
package systemdesign

import "sync"

// Design a Search Engine
// Implements a system design component for question #302.
type DesignASearchEngine struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignASearchEngine creates a new system component.
func NewDesignASearchEngine() *DesignASearchEngine {
        return &DesignASearchEngine{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignASearchEngine) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignASearchEngine) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignASearchEngine) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
