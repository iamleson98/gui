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
type JitVsInterpreterPerformance struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewJitVsInterpreterPerformance creates a new performance optimizer.
func NewJitVsInterpreterPerformance() *JitVsInterpreterPerformance {
        return &JitVsInterpreterPerformance{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *JitVsInterpreterPerformance) Get(key uint64) (interface{, bool) {
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
func (p *JitVsInterpreterPerformance) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *JitVsInterpreterPerformance) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
