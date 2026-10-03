// Question #326: Design a Workflow Engine
// Category: System Design | Difficulty: Hard
// Concepts: workflow, durable, retries, timers
// Description: Design a durable workflow engine (Temporal/Airflow-like) with retries, timers, and state.
package systemdesign

import "sync"

// Design a Workflow Engine
// Implements a system design component for question #326.
type Q326_DesignAWorkflowEngine struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ326_DesignAWorkflowEngine creates a new system component.
func NewQ326_DesignAWorkflowEngine() *Q326_DesignAWorkflowEngine {
        return &Q326_DesignAWorkflowEngine{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q326_DesignAWorkflowEngine) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q326_DesignAWorkflowEngine) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q326_DesignAWorkflowEngine) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
