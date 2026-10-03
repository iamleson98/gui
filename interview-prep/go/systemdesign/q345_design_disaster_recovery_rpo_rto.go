// Question #345: Design Disaster Recovery (RPO/RTO)
// Category: System Design | Difficulty: Hard
// Concepts: DR, RPO, RTO, failover
// Description: Quantify RPO/RTO and design backup, replication, and failover to meet them.
package systemdesign

import "sync"

// Design Disaster Recovery (RPO/RTO)
// Implements a system design component for question #345.
type Q345_DesignDisasterRecoveryRpoRto struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ345_DesignDisasterRecoveryRpoRto creates a new system component.
func NewQ345_DesignDisasterRecoveryRpoRto() *Q345_DesignDisasterRecoveryRpoRto {
        return &Q345_DesignDisasterRecoveryRpoRto{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q345_DesignDisasterRecoveryRpoRto) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q345_DesignDisasterRecoveryRpoRto) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q345_DesignDisasterRecoveryRpoRto) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
