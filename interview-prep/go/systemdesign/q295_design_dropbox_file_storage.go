// Question #295: Design Dropbox / File Storage
// Category: System Design | Difficulty: Hard
// Concepts: file sync, chunking, dedup, conflict
// Description: Design a file-sync service with chunking, deduplication, and conflict resolution.
package systemdesign

import "sync"

// Design Dropbox / File Storage
// Implements a system design component for question #295.
type DesignDropboxFileStorage struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignDropboxFileStorage creates a new system component.
func NewDesignDropboxFileStorage() *DesignDropboxFileStorage {
        return &DesignDropboxFileStorage{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignDropboxFileStorage) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignDropboxFileStorage) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignDropboxFileStorage) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
