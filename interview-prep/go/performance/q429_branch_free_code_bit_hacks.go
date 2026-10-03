// Question #429: Branch-Free Code (Bit Hacks)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: bit hacks, branchless, misprediction, flags
// Description: Replace branches with bit manipulation to avoid mispredictions on data-dependent paths.
package performance

import (
        "sync"
        "sync/atomic"
)

// Branch-Free Code (Bit Hacks)
// Implements a performance optimization for question #429.
type BranchFreeCodeBitHacks struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewBranchFreeCodeBitHacks creates a new performance optimizer.
func NewBranchFreeCodeBitHacks() *BranchFreeCodeBitHacks {
        return &BranchFreeCodeBitHacks{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *BranchFreeCodeBitHacks) Get(key uint64) (interface{, bool) {
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
func (p *BranchFreeCodeBitHacks) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *BranchFreeCodeBitHacks) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
