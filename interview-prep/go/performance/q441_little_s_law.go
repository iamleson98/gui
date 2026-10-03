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
type Q441_LittleSLaw struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ441_LittleSLaw creates a new performance optimizer.
func NewQ441_LittleSLaw() *Q441_LittleSLaw {
        return &Q441_LittleSLaw{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q441_LittleSLaw) Get(key uint64) (any, bool) {
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
func (p *Q441_LittleSLaw) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q441_LittleSLaw) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
