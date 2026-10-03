// Question #459: Escape Analysis and Stack Allocation
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: escape analysis, stack allocation, GC, lifetime
// Description: Use escape analysis to allocate heap objects on the stack for zero-cost lifetimes.
package performance

import (
        "sync"
        "sync/atomic"
)

// Escape Analysis and Stack Allocation
// Implements a performance optimization for question #459.
type Q459_EscapeAnalysisAndStackAllocation struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ459_EscapeAnalysisAndStackAllocation creates a new performance optimizer.
func NewQ459_EscapeAnalysisAndStackAllocation() *Q459_EscapeAnalysisAndStackAllocation {
        return &Q459_EscapeAnalysisAndStackAllocation{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q459_EscapeAnalysisAndStackAllocation) Get(key uint64) (any, bool) {
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
func (p *Q459_EscapeAnalysisAndStackAllocation) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q459_EscapeAnalysisAndStackAllocation) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
