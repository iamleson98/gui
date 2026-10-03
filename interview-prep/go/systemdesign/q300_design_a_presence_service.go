// Question #300: Design a Presence Service
// Category: System Design | Difficulty: Hard
// Concepts: presence, heartbeat, pub/sub, fan-out
// Description: Design a presence service tracking online status with heartbeats and a pub/sub fan-out.
package systemdesign

import "sync"

// Design a Presence Service
// Implements a system design component for question #300.
type DesignAPresenceService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAPresenceService creates a new system component.
func NewDesignAPresenceService() *DesignAPresenceService {
        return &DesignAPresenceService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAPresenceService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAPresenceService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAPresenceService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
