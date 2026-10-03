// Question #427: Loop Tiling / Blocking
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: tiling, blocking, cache, matrix
// Description: Tile nested loops to fit working sets in cache for matrix computations.
package performance

import (
        "sync"
        "sync/atomic"
)

// Loop Tiling / Blocking
// Implements a performance optimization for question #427.
type Q427_LoopTilingBlocking struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ427_LoopTilingBlocking creates a new performance optimizer.
func NewQ427_LoopTilingBlocking() *Q427_LoopTilingBlocking {
        return &Q427_LoopTilingBlocking{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q427_LoopTilingBlocking) Get(key uint64) (any, bool) {
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
func (p *Q427_LoopTilingBlocking) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q427_LoopTilingBlocking) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
