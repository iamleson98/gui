// Question #431: Latency Numbers Every Programmer Should Know
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: latency, L1, DRAM, network
// Description: Reason about L1, DRAM, SSD, and network latencies to design fast systems.
package performance

import (
        "sync"
        "sync/atomic"
)

// Latency Numbers Every Programmer Should Know
// Implements a performance optimization for question #431.
type Q431_LatencyNumbersEveryProgrammerShouldKnow struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ431_LatencyNumbersEveryProgrammerShouldKnow creates a new performance optimizer.
func NewQ431_LatencyNumbersEveryProgrammerShouldKnow() *Q431_LatencyNumbersEveryProgrammerShouldKnow {
        return &Q431_LatencyNumbersEveryProgrammerShouldKnow{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q431_LatencyNumbersEveryProgrammerShouldKnow) Get(key uint64) (any, bool) {
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
func (p *Q431_LatencyNumbersEveryProgrammerShouldKnow) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q431_LatencyNumbersEveryProgrammerShouldKnow) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
