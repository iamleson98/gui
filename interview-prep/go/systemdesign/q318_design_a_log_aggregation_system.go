// Question #318: Design a Log Aggregation System
// Category: System Design | Difficulty: Hard
// Concepts: logs, ingestion, indexing, retention
// Description: Design a log pipeline with ingestion, indexing, retention, and query.
package systemdesign

import "sync"

// Design a Log Aggregation System
// Implements a system design component for question #318.
type Q318_DesignALogAggregationSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ318_DesignALogAggregationSystem creates a new system component.
func NewQ318_DesignALogAggregationSystem() *Q318_DesignALogAggregationSystem {
        return &Q318_DesignALogAggregationSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q318_DesignALogAggregationSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q318_DesignALogAggregationSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q318_DesignALogAggregationSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
