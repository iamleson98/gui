// Question #357: Design Context Propagation (W3C Trace Context)
// Category: System Design | Difficulty: Hard
// Concepts: trace context, W3C, propagation, headers
// Description: Propagate trace context across process boundaries with W3C headers.
package systemdesign

import "sync"

// Design Context Propagation (W3C Trace Context)
// Implements a system design component for question #357.
type Q357_DesignContextPropagationW3CTraceContext struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ357_DesignContextPropagationW3CTraceContext creates a new system component.
func NewQ357_DesignContextPropagationW3CTraceContext() *Q357_DesignContextPropagationW3CTraceContext {
        return &Q357_DesignContextPropagationW3CTraceContext{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q357_DesignContextPropagationW3CTraceContext) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q357_DesignContextPropagationW3CTraceContext) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q357_DesignContextPropagationW3CTraceContext) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
