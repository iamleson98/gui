// Question #346: Design a Deduplication Service
// Category: System Design | Difficulty: Hard
// Concepts: dedup, content hash, windowed, idempotency
// Description: Design a service that deduplicates events using content hashing and a windowed store.
package systemdesign

import "sync"

// Design a Deduplication Service
// Implements a system design component for question #346.
type DesignADeduplicationService struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignADeduplicationService creates a new system component.
func NewDesignADeduplicationService() *DesignADeduplicationService {
        return &DesignADeduplicationService{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignADeduplicationService) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignADeduplicationService) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignADeduplicationService) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
