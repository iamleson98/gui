// Question #413: perf / perf_events
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: perf, perf_events, PMU, counters
// Description: Use perf to capture hardware counters, branch misses, and cache misses.
package performance

import (
        "sync"
        "sync/atomic"
)

// perf / perf_events
// Implements a performance optimization for question #413.
type PerfPerfEvents struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewPerfPerfEvents creates a new performance optimizer.
func NewPerfPerfEvents() *PerfPerfEvents {
        return &PerfPerfEvents{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *PerfPerfEvents) Get(key uint64) (interface{, bool) {
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
func (p *PerfPerfEvents) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *PerfPerfEvents) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
