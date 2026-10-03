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
type CacheLinesAndPrefetching struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewCacheLinesAndPrefetching creates a new performance optimizer.
func NewCacheLinesAndPrefetching() *CacheLinesAndPrefetching {
        return &CacheLinesAndPrefetching{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *CacheLinesAndPrefetching) Get(key uint64) (interface{, bool) {
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
func (p *CacheLinesAndPrefetching) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *CacheLinesAndPrefetching) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
