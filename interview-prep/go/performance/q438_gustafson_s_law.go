// Question #438: Gustafson's Law
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: Gustafson, scaled speedup, parallel, problem size
// Description: Apply Gustafson's law to scale problems with processors rather than fix them.
package performance

import (
        "sync"
        "sync/atomic"
)

// Gustafson's Law
// Implements a performance optimization for question #438.
type GustafsonSLaw struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewGustafsonSLaw creates a new performance optimizer.
func NewGustafsonSLaw() *GustafsonSLaw {
        return &GustafsonSLaw{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *GustafsonSLaw) Get(key uint64) (interface{, bool) {
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
func (p *GustafsonSLaw) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *GustafsonSLaw) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
