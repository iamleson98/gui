// Question #419: Superscalar and Out-of-Order Execution
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: superscalar, OoO, ILP, renaming
// Description: Explain how superscalar and out-of-order execution expose instruction-level parallelism.
package performance

import (
        "sync"
        "sync/atomic"
)

// Superscalar and Out-of-Order Execution
// Implements a performance optimization for question #419.
type Q419_SuperscalarAndOutOfOrderExecution struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ419_SuperscalarAndOutOfOrderExecution creates a new performance optimizer.
func NewQ419_SuperscalarAndOutOfOrderExecution() *Q419_SuperscalarAndOutOfOrderExecution {
        return &Q419_SuperscalarAndOutOfOrderExecution{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q419_SuperscalarAndOutOfOrderExecution) Get(key uint64) (any, bool) {
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
func (p *Q419_SuperscalarAndOutOfOrderExecution) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q419_SuperscalarAndOutOfOrderExecution) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
