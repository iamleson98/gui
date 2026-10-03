// Question #458: JIT vs Interpreter Performance
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: JIT, interpreter, compilation, warmup
// Description: Compare JIT compilation with interpretation and where each pays off.
package performance

import (
        "sync"
        "sync/atomic"
)

// JIT vs Interpreter Performance
// Implements a performance optimization for question #458.
type Q458_JitVsInterpreterPerformance struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ458_JitVsInterpreterPerformance creates a new performance optimizer.
func NewQ458_JitVsInterpreterPerformance() *Q458_JitVsInterpreterPerformance {
        return &Q458_JitVsInterpreterPerformance{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q458_JitVsInterpreterPerformance) Get(key uint64) (any, bool) {
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
func (p *Q458_JitVsInterpreterPerformance) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q458_JitVsInterpreterPerformance) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
