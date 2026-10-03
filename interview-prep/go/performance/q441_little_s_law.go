// Question #441: Little's Law
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: Little's Law, concurrency, queue, throughput
// Description: Apply Little's Law (L = lambda * W) to size queues and concurrency.
package performance

import (
        "sync"
        "sync/atomic"
)

// Little's Law
// Implements a performance optimization for question #441.
type LittleSLaw struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewLittleSLaw creates a new performance optimizer.
func NewLittleSLaw() *LittleSLaw {
        return &LittleSLaw{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *LittleSLaw) Get(key uint64) (interface{, bool) {
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
func (p *LittleSLaw) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *LittleSLaw) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
