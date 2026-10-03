// Question #341: Design a Config Service
// Category: System Design | Difficulty: Hard
// Concepts: config, dynamic, watch, versioning
// Description: Design a dynamic configuration service with watch notifications and versioning.
package systemdesign

import "sync"

// Design a Config Service
// Implements a system design component for question #341.
type DesignAConfigService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignAConfigService creates a new system component.
func NewDesignAConfigService() *DesignAConfigService {
        return &DesignAConfigService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignAConfigService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignAConfigService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignAConfigService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
