// Question #423: Data-Oriented Design
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: DOD, hot/cold split, cache efficiency, layout
// Description: Restructure data for cache efficiency by separating hot and cold fields.
package performance

import (
        "sync"
        "sync/atomic"
)

// Data-Oriented Design
// Implements a performance optimization for question #423.
type DataOrientedDesign struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewDataOrientedDesign creates a new performance optimizer.
func NewDataOrientedDesign() *DataOrientedDesign {
        return &DataOrientedDesign{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *DataOrientedDesign) Get(key uint64) (interface{, bool) {
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
func (p *DataOrientedDesign) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *DataOrientedDesign) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
