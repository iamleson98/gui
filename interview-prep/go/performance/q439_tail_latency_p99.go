// Question #439: Tail Latency (P99)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: tail latency, P99, straggler, fan-out
// Description: Measure and reduce P99 latency by eliminating stragglers in fan-out services.
package performance

import (
        "sync"
        "sync/atomic"
)

// Tail Latency (P99)
// Implements a performance optimization for question #439.
type Q439_TailLatencyP99 struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ439_TailLatencyP99 creates a new performance optimizer.
func NewQ439_TailLatencyP99() *Q439_TailLatencyP99 {
        return &Q439_TailLatencyP99{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q439_TailLatencyP99) Get(key uint64) (any, bool) {
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
func (p *Q439_TailLatencyP99) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q439_TailLatencyP99) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
