// Question #306: Design Geospatial Indexing (Geohash/Quadtree)
// Category: System Design | Difficulty: Hard
// Concepts: geospatial, geohash, quadtree, nearby
// Description: Index moving vehicles by geohash or quadtree for nearby-driver queries.
package systemdesign

import "sync"

// Design Geospatial Indexing (Geohash/Quadtree)
// Implements a system design component for question #306.
type Q306_DesignGeospatialIndexingGeohashQuadtree struct {
        mu      sync.RWMutex
        config  map[string]string
        metrics map[string]int64
}

// NewQ306_DesignGeospatialIndexingGeohashQuadtree creates a new system component.
func NewQ306_DesignGeospatialIndexingGeohashQuadtree() *Q306_DesignGeospatialIndexingGeohashQuadtree {
        return &Q306_DesignGeospatialIndexingGeohashQuadtree{
                config:  make(map[string]string),
                metrics: make(map[string]int64),
        }
}

// SetConfig updates a configuration value.
func (s *Q306_DesignGeospatialIndexingGeohashQuadtree) SetConfig(key, val string) {
        s.mu.Lock()
        s.config[key] = val
        s.mu.Unlock()
}

// GetConfig reads a configuration value.
func (s *Q306_DesignGeospatialIndexingGeohashQuadtree) GetConfig(key string) (string, bool) {
        s.mu.RLock()
        v, ok := s.config[key]
        s.mu.RUnlock()
        return v, ok
}

// IncrementMetric increments a metric counter.
func (s *Q306_DesignGeospatialIndexingGeohashQuadtree) IncrementMetric(key string) {
        s.mu.Lock()
        s.metrics[key]++
        s.mu.Unlock()
}
