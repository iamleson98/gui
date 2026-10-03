// Question #456: Cache Miss Profiling
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: cache miss, profiling, PMC, layout
// Description: Measure L1/L2/L3 and LLC misses to find cache-unfriendly data layouts.
package performance

import (
        "sync"
        "sync/atomic"
)

// Cache Miss Profiling
// Implements a performance optimization for question #456.
type CacheMissProfiling struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewCacheMissProfiling creates a new performance optimizer.
func NewCacheMissProfiling() *CacheMissProfiling {
        return &CacheMissProfiling{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *CacheMissProfiling) Get(key uint64) (interface{, bool) {
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
func (p *CacheMissProfiling) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *CacheMissProfiling) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
