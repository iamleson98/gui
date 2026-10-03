// Question #314: Design a Leaderboard (Sorted Sets)
// Category: System Design | Difficulty: Hard
// Concepts: leaderboard, sorted sets, sharding, rollup
// Description: Design a global leaderboard using Redis sorted sets with sharded rollups.
package systemdesign

import "sync"

// Design a Leaderboard (Sorted Sets)
// Implements a system design component for question #314.
type Q314_DesignALeaderboardSortedSets struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ314_DesignALeaderboardSortedSets creates a new system component.
func NewQ314_DesignALeaderboardSortedSets() *Q314_DesignALeaderboardSortedSets {
        return &Q314_DesignALeaderboardSortedSets{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q314_DesignALeaderboardSortedSets) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q314_DesignALeaderboardSortedSets) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q314_DesignALeaderboardSortedSets) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
