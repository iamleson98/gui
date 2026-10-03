// Question #446: io_uring vs epoll
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: io_uring, epoll, submission queue, completion
// Description: Contrast io_uring's submission/completion queues with epoll's readiness model.
package performance

import (
        "sync"
        "sync/atomic"
)

// io_uring vs epoll
// Implements a performance optimization for question #446.
type Q446_IoUringVsEpoll struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ446_IoUringVsEpoll creates a new performance optimizer.
func NewQ446_IoUringVsEpoll() *Q446_IoUringVsEpoll {
        return &Q446_IoUringVsEpoll{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q446_IoUringVsEpoll) Get(key uint64) (any, bool) {
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
func (p *Q446_IoUringVsEpoll) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q446_IoUringVsEpoll) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
