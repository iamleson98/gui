// Question #435: Memory-Bound vs Compute-Bound
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: memory-bound, compute-bound, roofline, classification
// Description: Classify code as memory- or compute-bound to pick the right optimization.
package performance

import (
        "sync"
        "sync/atomic"
)

// Memory-Bound vs Compute-Bound
// Implements a performance optimization for question #435.
type MemoryBoundVsComputeBound struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewMemoryBoundVsComputeBound creates a new performance optimizer.
func NewMemoryBoundVsComputeBound() *MemoryBoundVsComputeBound {
        return &MemoryBoundVsComputeBound{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *MemoryBoundVsComputeBound) Get(key uint64) (interface{, bool) {
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
func (p *MemoryBoundVsComputeBound) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *MemoryBoundVsComputeBound) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
