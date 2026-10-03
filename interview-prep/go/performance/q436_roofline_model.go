// Question #436: Roofline Model
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: roofline, arithmetic intensity, peak, bandwidth
// Description: Plot the roofline model to see whether arithmetic intensity caps performance.
package performance

import (
        "sync"
        "sync/atomic"
)

// Roofline Model
// Implements a performance optimization for question #436.
type RooflineModel struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewRooflineModel creates a new performance optimizer.
func NewRooflineModel() *RooflineModel {
        return &RooflineModel{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *RooflineModel) Get(key uint64) (interface{, bool) {
        p.mu.Lock()
        v, ok := p.cache[key]
        p.mu.Unlock()
        if ok {
                p.hits.Add(1)
        } else {
                p.misses.Add(1)
        }
        return v, ok
}

// Set stores a value in the cache.
func (p *RooflineModel) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *RooflineModel) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
