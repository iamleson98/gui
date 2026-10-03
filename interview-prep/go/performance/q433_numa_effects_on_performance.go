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
type Q433_NumaEffectsOnPerformance struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ433_NumaEffectsOnPerformance creates a new performance optimizer.
func NewQ433_NumaEffectsOnPerformance() *Q433_NumaEffectsOnPerformance {
        return &Q433_NumaEffectsOnPerformance{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q433_NumaEffectsOnPerformance) Get(key uint64) (any, bool) {
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
func (p *Q433_NumaEffectsOnPerformance) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q433_NumaEffectsOnPerformance) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
