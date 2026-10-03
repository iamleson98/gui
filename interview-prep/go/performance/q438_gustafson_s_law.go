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
type Q438_GustafsonSLaw struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ438_GustafsonSLaw creates a new performance optimizer.
func NewQ438_GustafsonSLaw() *Q438_GustafsonSLaw {
        return &Q438_GustafsonSLaw{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q438_GustafsonSLaw) Get(key uint64) (any, bool) {
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
func (p *Q438_GustafsonSLaw) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q438_GustafsonSLaw) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
