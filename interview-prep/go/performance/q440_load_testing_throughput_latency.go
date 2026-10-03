// Question #440: Load Testing (Throughput/Latency)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: load testing, throughput, latency, sustained
// Description: Design load tests that report throughput-latency curves under sustained load.
package performance

import (
        "sync"
        "sync/atomic"
)

// Load Testing (Throughput/Latency)
// Implements a performance optimization for question #440.
type Q440_LoadTestingThroughputLatency struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ440_LoadTestingThroughputLatency creates a new performance optimizer.
func NewQ440_LoadTestingThroughputLatency() *Q440_LoadTestingThroughputLatency {
        return &Q440_LoadTestingThroughputLatency{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q440_LoadTestingThroughputLatency) Get(key uint64) (any, bool) {
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
func (p *Q440_LoadTestingThroughputLatency) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q440_LoadTestingThroughputLatency) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
