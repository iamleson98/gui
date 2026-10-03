// Question #301: Design a Notification Service
// Category: System Design | Difficulty: Hard
// Concepts: notifications, fan-out, dedup, preferences
// Description: Design a multi-channel notification fan-out with dedup, batching, and user preferences.
package systemdesign

import "sync"

// Design a Notification Service
// Implements a system design component for question #301.
type Q301_DesignANotificationService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ301_DesignANotificationService creates a new system component.
func NewQ301_DesignANotificationService() *Q301_DesignANotificationService {
        return &Q301_DesignANotificationService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q301_DesignANotificationService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q301_DesignANotificationService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q301_DesignANotificationService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
