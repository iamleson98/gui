// Question #412: Flame Graphs
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: flame graph, stack samples, hot path, visualization
// Description: Build and read flame graphs to identify hot code paths from stack samples.
package performance

import (
        "sync"
        "sync/atomic"
)

// Flame Graphs
// Implements a performance optimization for question #412.
type FlameGraphs struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewFlameGraphs creates a new performance optimizer.
func NewFlameGraphs() *FlameGraphs {
        return &FlameGraphs{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *FlameGraphs) Get(key uint64) (interface{, bool) {
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
func (p *FlameGraphs) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *FlameGraphs) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
