// Question #300: Design a Presence Service
// Category: System Design | Difficulty: Hard
// Concepts: presence, heartbeat, pub/sub, fan-out
// Description: Design a presence service tracking online status with heartbeats and a pub/sub fan-out.
package systemdesign

import "sync"

// Design a Presence Service
// Implements a system design component for question #300.
type Q300_DesignAPresenceService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ300_DesignAPresenceService creates a new system component.
func NewQ300_DesignAPresenceService() *Q300_DesignAPresenceService {
        return &Q300_DesignAPresenceService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q300_DesignAPresenceService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q300_DesignAPresenceService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q300_DesignAPresenceService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
