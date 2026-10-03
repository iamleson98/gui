// Question #421: Cache Lines and Prefetching
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: cache line, prefetch, locality, size
// Description: Size data accesses to cache lines and exploit hardware prefetching.
package performance

import (
        "sync"
        "sync/atomic"
)

// Cache Lines and Prefetching
// Implements a performance optimization for question #421.
type Q421_CacheLinesAndPrefetching struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ421_CacheLinesAndPrefetching creates a new performance optimizer.
func NewQ421_CacheLinesAndPrefetching() *Q421_CacheLinesAndPrefetching {
        return &Q421_CacheLinesAndPrefetching{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q421_CacheLinesAndPrefetching) Get(key uint64) (any, bool) {
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
func (p *Q421_CacheLinesAndPrefetching) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q421_CacheLinesAndPrefetching) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
