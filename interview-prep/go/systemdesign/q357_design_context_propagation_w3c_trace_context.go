// Question #357: Design Context Propagation (W3C Trace Context)
// Category: System Design | Difficulty: Hard
// Concepts: trace context, W3C, propagation, headers
// Description: Propagate trace context across process boundaries with W3C headers.
package systemdesign

import "sync"

// Design Context Propagation (W3C Trace Context)
// Implements a system design component for question #357.
type DesignContextPropagationW3CTraceContext struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignContextPropagationW3CTraceContext creates a new system component.
func NewDesignContextPropagationW3CTraceContext() *DesignContextPropagationW3CTraceContext {
        return &DesignContextPropagationW3CTraceContext{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignContextPropagationW3CTraceContext) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignContextPropagationW3CTraceContext) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignContextPropagationW3CTraceContext) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
