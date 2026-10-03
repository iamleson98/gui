// Question #450: I/O Scheduler (deadline/CFQ)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: I/O scheduler, deadline, CFQ, merging
// Description: Explain disk I/O schedulers and their effect on latency and throughput.
package performance

import (
        "sync"
        "sync/atomic"
)

// I/O Scheduler (deadline/CFQ)
// Implements a performance optimization for question #450.
type Q450_IOSchedulerDeadlineCfq struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ450_IOSchedulerDeadlineCfq creates a new performance optimizer.
func NewQ450_IOSchedulerDeadlineCfq() *Q450_IOSchedulerDeadlineCfq {
        return &Q450_IOSchedulerDeadlineCfq{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q450_IOSchedulerDeadlineCfq) Get(key uint64) (any, bool) {
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
func (p *Q450_IOSchedulerDeadlineCfq) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q450_IOSchedulerDeadlineCfq) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
