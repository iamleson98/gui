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
type LockContentionProfiling struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewLockContentionProfiling creates a new performance optimizer.
func NewLockContentionProfiling() *LockContentionProfiling {
        return &LockContentionProfiling{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *LockContentionProfiling) Get(key uint64) (interface{, bool) {
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
func (p *LockContentionProfiling) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *LockContentionProfiling) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
