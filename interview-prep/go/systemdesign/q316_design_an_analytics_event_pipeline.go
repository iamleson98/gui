// Question #316: Design an Analytics/Event Pipeline
// Category: System Design | Difficulty: Hard
// Concepts: analytics, Kafka, stream processing, warehouse
// Description: Design an event ingestion pipeline with Kafka, stream processing, and warehousing.
package systemdesign

import "sync"

// Design an Analytics/Event Pipeline
// Implements a system design component for question #316.
type Q316_DesignAnAnalyticsEventPipeline struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ316_DesignAnAnalyticsEventPipeline creates a new system component.
func NewQ316_DesignAnAnalyticsEventPipeline() *Q316_DesignAnAnalyticsEventPipeline {
        return &Q316_DesignAnAnalyticsEventPipeline{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q316_DesignAnAnalyticsEventPipeline) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q316_DesignAnAnalyticsEventPipeline) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q316_DesignAnAnalyticsEventPipeline) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
