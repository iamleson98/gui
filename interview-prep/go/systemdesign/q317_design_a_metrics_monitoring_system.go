// Question #317: Design a Metrics/Monitoring System
// Category: System Design | Difficulty: Hard
// Concepts: metrics, time-series, cardinality, downsampling
// Description: Design a time-series metrics system with cardinality control and downsampling.
package systemdesign

import "sync"

// Design a Metrics/Monitoring System
// Implements a system design component for question #317.
type Q317_DesignAMetricsMonitoringSystem struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ317_DesignAMetricsMonitoringSystem creates a new system component.
func NewQ317_DesignAMetricsMonitoringSystem() *Q317_DesignAMetricsMonitoringSystem {
        return &Q317_DesignAMetricsMonitoringSystem{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q317_DesignAMetricsMonitoringSystem) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q317_DesignAMetricsMonitoringSystem) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q317_DesignAMetricsMonitoringSystem) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
