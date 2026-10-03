// Question #317: Design a Metrics/Monitoring System
// Category: System Design | Difficulty: Hard
// Concepts: metrics, time-series, cardinality, downsampling
// Description: Design a time-series metrics system with cardinality control and downsampling.
package systemdesign

import "sync"

// Design a Metrics/Monitoring System
// Implements a system design component for question #317.
type DesignAMetricsMonitoringSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAMetricsMonitoringSystem creates a new system component.
func NewDesignAMetricsMonitoringSystem() *DesignAMetricsMonitoringSystem {
        return &DesignAMetricsMonitoringSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAMetricsMonitoringSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAMetricsMonitoringSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAMetricsMonitoringSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
