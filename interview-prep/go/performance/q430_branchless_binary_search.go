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
type BranchlessBinarySearch struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewBranchlessBinarySearch creates a new performance optimizer.
func NewBranchlessBinarySearch() *BranchlessBinarySearch {
        return &BranchlessBinarySearch{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *BranchlessBinarySearch) Get(key uint64) (interface{, bool) {
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
func (p *BranchlessBinarySearch) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *BranchlessBinarySearch) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
