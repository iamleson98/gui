// Question #353: Design Hinted Handoff (Dynamo)
// Category: System Design | Difficulty: Hard
// Concepts: hinted handoff, Dynamo, replica, recovery
// Description: Store writes for temporarily unavailable replicas and hand them off on recovery.
package systemdesign

import "sync"

// Design Hinted Handoff (Dynamo)
// Implements a system design component for question #353.
type DesignHintedHandoffDynamo struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignHintedHandoffDynamo creates a new system component.
func NewDesignHintedHandoffDynamo() *DesignHintedHandoffDynamo {
        return &DesignHintedHandoffDynamo{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignHintedHandoffDynamo) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignHintedHandoffDynamo) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignHintedHandoffDynamo) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
