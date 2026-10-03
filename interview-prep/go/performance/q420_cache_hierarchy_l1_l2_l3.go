// Question #420: Cache Hierarchy (L1/L2/L3)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: cache, L1/L2/L3, latency, bandwidth
// Description: Model the L1/L2/L3 cache hierarchy and quantify latency and bandwidth at each level.
package performance

import (
        "sync"
        "sync/atomic"
)

// Cache Hierarchy (L1/L2/L3)
// Implements a performance optimization for question #420.
type Q420_CacheHierarchyL1L2L3 struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ420_CacheHierarchyL1L2L3 creates a new performance optimizer.
func NewQ420_CacheHierarchyL1L2L3() *Q420_CacheHierarchyL1L2L3 {
        return &Q420_CacheHierarchyL1L2L3{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q420_CacheHierarchyL1L2L3) Get(key uint64) (any, bool) {
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
func (p *Q420_CacheHierarchyL1L2L3) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q420_CacheHierarchyL1L2L3) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
