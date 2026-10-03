// Question #442: Queueing Theory (M/M/1)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: queueing theory, M/M/1, utilization, latency
// Description: Model an M/M/1 queue to predict latency under arrival and service rate variation.
package performance

import (
        "sync"
        "sync/atomic"
)

// Queueing Theory (M/M/1)
// Implements a performance optimization for question #442.
type QueueingTheoryMM1 struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQueueingTheoryMM1 creates a new performance optimizer.
func NewQueueingTheoryMM1() *QueueingTheoryMM1 {
        return &QueueingTheoryMM1{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *QueueingTheoryMM1) Get(key uint64) (interface{, bool) {
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
func (p *QueueingTheoryMM1) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *QueueingTheoryMM1) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
