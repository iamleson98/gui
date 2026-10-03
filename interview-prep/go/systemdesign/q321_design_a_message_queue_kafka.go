// Question #321: Design a Message Queue (Kafka)
// Category: System Design | Difficulty: Hard
// Concepts: message queue, Kafka, partition, replication
// Description: Design a partitioned, replicated log-based message queue with consumer groups.
package systemdesign

import "sync"

// Design a Message Queue (Kafka)
// Implements a system design component for question #321.
type Q321_DesignAMessageQueueKafka struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ321_DesignAMessageQueueKafka creates a new system component.
func NewQ321_DesignAMessageQueueKafka() *Q321_DesignAMessageQueueKafka {
        return &Q321_DesignAMessageQueueKafka{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q321_DesignAMessageQueueKafka) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q321_DesignAMessageQueueKafka) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q321_DesignAMessageQueueKafka) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
