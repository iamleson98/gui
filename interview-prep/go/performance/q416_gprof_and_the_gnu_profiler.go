// Question #416: gprof and the GNU Profiler
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: gprof, call graph, flat profile, instrumentation
// Description: Use gprof call-graph and flat profiles to locate hot functions in C programs.
package performance

import (
        "sync"
        "sync/atomic"
)

// gprof and the GNU Profiler
// Implements a performance optimization for question #416.
type Q416_GprofAndTheGnuProfiler struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ416_GprofAndTheGnuProfiler creates a new performance optimizer.
func NewQ416_GprofAndTheGnuProfiler() *Q416_GprofAndTheGnuProfiler {
        return &Q416_GprofAndTheGnuProfiler{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q416_GprofAndTheGnuProfiler) Get(key uint64) (any, bool) {
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
func (p *Q416_GprofAndTheGnuProfiler) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q416_GprofAndTheGnuProfiler) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
