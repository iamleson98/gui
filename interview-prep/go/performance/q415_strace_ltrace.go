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
type StraceLtrace struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewStraceLtrace creates a new performance optimizer.
func NewStraceLtrace() *StraceLtrace {
        return &StraceLtrace{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *StraceLtrace) Get(key uint64) (interface{, bool) {
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
func (p *StraceLtrace) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *StraceLtrace) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
