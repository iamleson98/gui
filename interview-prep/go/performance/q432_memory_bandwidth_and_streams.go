// Question #432: Memory Bandwidth and Streams
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: bandwidth, STREAM, memory-bound, measurement
// Description: Measure memory bandwidth with the STREAM benchmark and detect bandwidth-bound code.
package performance

import (
        "sync"
        "sync/atomic"
)

// Memory Bandwidth and Streams
// Implements a performance optimization for question #432.
type MemoryBandwidthAndStreams struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewMemoryBandwidthAndStreams creates a new performance optimizer.
func NewMemoryBandwidthAndStreams() *MemoryBandwidthAndStreams {
        return &MemoryBandwidthAndStreams{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *MemoryBandwidthAndStreams) Get(key uint64) (interface{, bool) {
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
func (p *MemoryBandwidthAndStreams) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *MemoryBandwidthAndStreams) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
