// Question #301: Design a Notification Service
// Category: System Design | Difficulty: Hard
// Concepts: notifications, fan-out, dedup, preferences
// Description: Design a multi-channel notification fan-out with dedup, batching, and user preferences.
package systemdesign

import "sync"

// Design a Notification Service
// Implements a system design component for question #301.
type DesignANotificationService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignANotificationService creates a new system component.
func NewDesignANotificationService() *DesignANotificationService {
        return &DesignANotificationService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignANotificationService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignANotificationService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignANotificationService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
