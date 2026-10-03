// Question #437: Amdahl's Law
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: Amdahl, speedup, parallel fraction, bounds
// Description: Apply Amdahl's law to bound speedup from parallelizing a fraction of a program.
package performance

import (
        "sync"
        "sync/atomic"
)

// Amdahl's Law
// Implements a performance optimization for question #437.
type Q437_AmdahlSLaw struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ437_AmdahlSLaw creates a new performance optimizer.
func NewQ437_AmdahlSLaw() *Q437_AmdahlSLaw {
        return &Q437_AmdahlSLaw{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q437_AmdahlSLaw) Get(key uint64) (any, bool) {
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
func (p *Q437_AmdahlSLaw) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q437_AmdahlSLaw) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
