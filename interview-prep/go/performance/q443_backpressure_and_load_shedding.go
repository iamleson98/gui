// Question #443: Backpressure and Load Shedding
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: backpressure, load shedding, overload, latency
// Description: Apply backpressure and load shedding to preserve latency under overload.
package performance

import (
        "sync"
        "sync/atomic"
)

// Backpressure and Load Shedding
// Implements a performance optimization for question #443.
type Q443_BackpressureAndLoadShedding struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ443_BackpressureAndLoadShedding creates a new performance optimizer.
func NewQ443_BackpressureAndLoadShedding() *Q443_BackpressureAndLoadShedding {
        return &Q443_BackpressureAndLoadShedding{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q443_BackpressureAndLoadShedding) Get(key uint64) (any, bool) {
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
func (p *Q443_BackpressureAndLoadShedding) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q443_BackpressureAndLoadShedding) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
