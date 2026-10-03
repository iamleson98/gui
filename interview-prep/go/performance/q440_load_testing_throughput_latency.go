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
type LoadTestingThroughputLatency struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewLoadTestingThroughputLatency creates a new performance optimizer.
func NewLoadTestingThroughputLatency() *LoadTestingThroughputLatency {
        return &LoadTestingThroughputLatency{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *LoadTestingThroughputLatency) Get(key uint64) (interface{, bool) {
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
func (p *LoadTestingThroughputLatency) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *LoadTestingThroughputLatency) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
