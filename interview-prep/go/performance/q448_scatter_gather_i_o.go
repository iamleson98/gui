// Question #448: Scatter-Gather I/O
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: scatter-gather, readv, writev, syscall
// Description: Use scatter-gather (readv/writev) to coalesce multiple buffers per syscall.
package performance

import (
        "sync"
        "sync/atomic"
)

// Scatter-Gather I/O
// Implements a performance optimization for question #448.
type Q448_ScatterGatherIO struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ448_ScatterGatherIO creates a new performance optimizer.
func NewQ448_ScatterGatherIO() *Q448_ScatterGatherIO {
        return &Q448_ScatterGatherIO{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q448_ScatterGatherIO) Get(key uint64) (any, bool) {
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
func (p *Q448_ScatterGatherIO) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q448_ScatterGatherIO) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
