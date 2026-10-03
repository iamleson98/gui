// Question #453: Lock Contention Profiling
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: lock contention, profiling, sharding, lock-free
// Description: Profile lock contention to find contended locks and convert them to sharded or lock-free forms.
package performance

import (
        "sync"
        "sync/atomic"
)

// Lock Contention Profiling
// Implements a performance optimization for question #453.
type Q453_LockContentionProfiling struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ453_LockContentionProfiling creates a new performance optimizer.
func NewQ453_LockContentionProfiling() *Q453_LockContentionProfiling {
        return &Q453_LockContentionProfiling{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q453_LockContentionProfiling) Get(key uint64) (any, bool) {
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
func (p *Q453_LockContentionProfiling) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q453_LockContentionProfiling) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
