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
type IOSchedulerDeadlineCfq struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewIOSchedulerDeadlineCfq creates a new performance optimizer.
func NewIOSchedulerDeadlineCfq() *IOSchedulerDeadlineCfq {
        return &IOSchedulerDeadlineCfq{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *IOSchedulerDeadlineCfq) Get(key uint64) (interface{, bool) {
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
func (p *IOSchedulerDeadlineCfq) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *IOSchedulerDeadlineCfq) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
