// Question #415: strace / ltrace
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: strace, ltrace, syscall, library
// Description: Use strace and ltrace to attribute time spent in system and library calls.
package performance

import (
        "sync"
        "sync/atomic"
)

// strace / ltrace
// Implements a performance optimization for question #415.
type Q415_StraceLtrace struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ415_StraceLtrace creates a new performance optimizer.
func NewQ415_StraceLtrace() *Q415_StraceLtrace {
        return &Q415_StraceLtrace{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q415_StraceLtrace) Get(key uint64) (any, bool) {
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
func (p *Q415_StraceLtrace) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q415_StraceLtrace) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
