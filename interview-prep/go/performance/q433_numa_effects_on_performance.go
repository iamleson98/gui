// Question #433: NUMA Effects on Performance
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: NUMA, remote access, locality, sockets
// Description: Diagnose NUMA-local vs remote access penalties for multi-socket workloads.
package performance

import (
        "sync"
        "sync/atomic"
)

// NUMA Effects on Performance
// Implements a performance optimization for question #433.
type NumaEffectsOnPerformance struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewNumaEffectsOnPerformance creates a new performance optimizer.
func NewNumaEffectsOnPerformance() *NumaEffectsOnPerformance {
        return &NumaEffectsOnPerformance{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *NumaEffectsOnPerformance) Get(key uint64) (interface{, bool) {
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
func (p *NumaEffectsOnPerformance) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *NumaEffectsOnPerformance) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
