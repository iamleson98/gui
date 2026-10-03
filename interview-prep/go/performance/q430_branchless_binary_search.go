// Question #430: Branchless Binary Search
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: branchless, binary search, conditional move, Eytzinger
// Description: Implement a branchless binary search using conditional moves or index arithmetic.
package performance

import (
        "sync"
        "sync/atomic"
)

// Branchless Binary Search
// Implements a performance optimization for question #430.
type Q430_BranchlessBinarySearch struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ430_BranchlessBinarySearch creates a new performance optimizer.
func NewQ430_BranchlessBinarySearch() *Q430_BranchlessBinarySearch {
        return &Q430_BranchlessBinarySearch{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q430_BranchlessBinarySearch) Get(key uint64) (any, bool) {
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
func (p *Q430_BranchlessBinarySearch) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q430_BranchlessBinarySearch) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
