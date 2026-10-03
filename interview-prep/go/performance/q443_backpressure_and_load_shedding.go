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
type BackpressureAndLoadShedding struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewBackpressureAndLoadShedding creates a new performance optimizer.
func NewBackpressureAndLoadShedding() *BackpressureAndLoadShedding {
        return &BackpressureAndLoadShedding{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *BackpressureAndLoadShedding) Get(key uint64) (interface{, bool) {
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
func (p *BackpressureAndLoadShedding) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *BackpressureAndLoadShedding) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
