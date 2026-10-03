// Question #356: Design a Distributed Tracing System
// Category: System Design | Difficulty: Hard
// Concepts: tracing, spans, sampling, context
// Description: Design an OpenTelemetry-style tracing system with spans, sampling, and context propagation.
package systemdesign

import "sync"

// Design a Distributed Tracing System
// Implements a system design component for question #356.
type DesignADistributedTracingSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignADistributedTracingSystem creates a new system component.
func NewDesignADistributedTracingSystem() *DesignADistributedTracingSystem {
        return &DesignADistributedTracingSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignADistributedTracingSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignADistributedTracingSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignADistributedTracingSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
