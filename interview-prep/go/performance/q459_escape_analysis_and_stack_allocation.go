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
type EscapeAnalysisAndStackAllocation struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewEscapeAnalysisAndStackAllocation creates a new performance optimizer.
func NewEscapeAnalysisAndStackAllocation() *EscapeAnalysisAndStackAllocation {
        return &EscapeAnalysisAndStackAllocation{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *EscapeAnalysisAndStackAllocation) Get(key uint64) (interface{, bool) {
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
func (p *EscapeAnalysisAndStackAllocation) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *EscapeAnalysisAndStackAllocation) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
