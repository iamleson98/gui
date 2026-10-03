// Question #306: Design Geospatial Indexing (Geohash/Quadtree)
// Category: System Design | Difficulty: Hard
// Concepts: geospatial, geohash, quadtree, nearby
// Description: Index moving vehicles by geohash or quadtree for nearby-driver queries.
package systemdesign

import "sync"

// Design Geospatial Indexing (Geohash/Quadtree)
// Implements a system design component for question #306.
type DesignGeospatialIndexingGeohashQuadtree struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewDesignGeospatialIndexingGeohashQuadtree creates a new system component.
func NewDesignGeospatialIndexingGeohashQuadtree() *DesignGeospatialIndexingGeohashQuadtree {
        return &DesignGeospatialIndexingGeohashQuadtree{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *DesignGeospatialIndexingGeohashQuadtree) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *DesignGeospatialIndexingGeohashQuadtree) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *DesignGeospatialIndexingGeohashQuadtree) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
