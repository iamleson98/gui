// Question #326: Design a Workflow Engine
// Category: System Design | Difficulty: Hard
// Concepts: workflow, durable, retries, timers
// Description: Design a durable workflow engine (Temporal/Airflow-like) with retries, timers, and state.
package systemdesign

import "sync"

// Design a Workflow Engine
// Implements a system design component for question #326.
type DesignAWorkflowEngine struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAWorkflowEngine creates a new system component.
func NewDesignAWorkflowEngine() *DesignAWorkflowEngine {
        return &DesignAWorkflowEngine{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAWorkflowEngine) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAWorkflowEngine) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAWorkflowEngine) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
