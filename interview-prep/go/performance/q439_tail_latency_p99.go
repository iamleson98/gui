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
type TailLatencyP99 struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewTailLatencyP99 creates a new performance optimizer.
func NewTailLatencyP99() *TailLatencyP99 {
        return &TailLatencyP99{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *TailLatencyP99) Get(key uint64) (interface{, bool) {
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
func (p *TailLatencyP99) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *TailLatencyP99) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
