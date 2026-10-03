// Question #426: Loop Unrolling
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: unrolling, ILP, loop overhead, code size
// Description: Unroll loops to reduce loop overhead and expose ILP, balancing code-size costs.
package performance

import (
        "sync"
        "sync/atomic"
)

// Loop Unrolling
// Implements a performance optimization for question #426.
type Q426_LoopUnrolling struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ426_LoopUnrolling creates a new performance optimizer.
func NewQ426_LoopUnrolling() *Q426_LoopUnrolling {
        return &Q426_LoopUnrolling{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q426_LoopUnrolling) Get(key uint64) (any, bool) {
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
func (p *Q426_LoopUnrolling) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q426_LoopUnrolling) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
