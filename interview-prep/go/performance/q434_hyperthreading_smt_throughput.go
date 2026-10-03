// Question #434: Hyperthreading/SMT Throughput
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: SMT, hyperthreading, throughput, contention
// Description: Reason about SMT throughput gains and contention on shared execution resources.
package performance

import (
        "sync"
        "sync/atomic"
)

// Hyperthreading/SMT Throughput
// Implements a performance optimization for question #434.
type Q434_HyperthreadingSmtThroughput struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ434_HyperthreadingSmtThroughput creates a new performance optimizer.
func NewQ434_HyperthreadingSmtThroughput() *Q434_HyperthreadingSmtThroughput {
        return &Q434_HyperthreadingSmtThroughput{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q434_HyperthreadingSmtThroughput) Get(key uint64) (any, bool) {
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
func (p *Q434_HyperthreadingSmtThroughput) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q434_HyperthreadingSmtThroughput) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
