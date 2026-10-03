// Question #296: Design Google Drive Collaborative Editing
// Category: System Design | Difficulty: Hard
// Concepts: collaborative editing, OT, CRDT, conflict
// Description: Design real-time collaborative editing using operational transformation or CRDTs.
package systemdesign

import "sync"

// Design Google Drive Collaborative Editing
// Implements a system design component for question #296.
type DesignGoogleDriveCollaborativeEditing struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignGoogleDriveCollaborativeEditing creates a new system component.
func NewDesignGoogleDriveCollaborativeEditing() *DesignGoogleDriveCollaborativeEditing {
        return &DesignGoogleDriveCollaborativeEditing{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignGoogleDriveCollaborativeEditing) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignGoogleDriveCollaborativeEditing) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignGoogleDriveCollaborativeEditing) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
