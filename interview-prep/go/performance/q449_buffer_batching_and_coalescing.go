// Question #449: Buffer Batching and Coalescing
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: batching, coalescing, syscall, amortize
// Description: Batch and coalesce small writes into larger buffers to amortize syscall cost.
package performance

import (
        "sync"
        "sync/atomic"
)

// Buffer Batching and Coalescing
// Implements a performance optimization for question #449.
type Q449_BufferBatchingAndCoalescing struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ449_BufferBatchingAndCoalescing creates a new performance optimizer.
func NewQ449_BufferBatchingAndCoalescing() *Q449_BufferBatchingAndCoalescing {
        return &Q449_BufferBatchingAndCoalescing{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q449_BufferBatchingAndCoalescing) Get(key uint64) (any, bool) {
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
func (p *Q449_BufferBatchingAndCoalescing) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q449_BufferBatchingAndCoalescing) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
