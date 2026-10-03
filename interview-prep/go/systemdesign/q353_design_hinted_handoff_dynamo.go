// Question #353: Design Hinted Handoff (Dynamo)
// Category: System Design | Difficulty: Hard
// Concepts: hinted handoff, Dynamo, replica, recovery
// Description: Store writes for temporarily unavailable replicas and hand them off on recovery.
package systemdesign

import "sync"

// Design Hinted Handoff (Dynamo)
// Implements a system design component for question #353.
type Q353_DesignHintedHandoffDynamo struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ353_DesignHintedHandoffDynamo creates a new system component.
func NewQ353_DesignHintedHandoffDynamo() *Q353_DesignHintedHandoffDynamo {
        return &Q353_DesignHintedHandoffDynamo{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q353_DesignHintedHandoffDynamo) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q353_DesignHintedHandoffDynamo) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q353_DesignHintedHandoffDynamo) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
