// Question #322: Design Kafka Log Structure
// Category: System Design | Difficulty: Hard
// Concepts: Kafka log, segments, index, retention
// Description: Explain Kafka's append-only segmented log with indexes and retention by size/time.
package systemdesign

import "sync"

// Design Kafka Log Structure
// Implements a system design component for question #322.
type DesignKafkaLogStructure struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignKafkaLogStructure creates a new system component.
func NewDesignKafkaLogStructure() *DesignKafkaLogStructure {
        return &DesignKafkaLogStructure{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignKafkaLogStructure) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignKafkaLogStructure) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignKafkaLogStructure) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
