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
type IoUringVsEpoll struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewIoUringVsEpoll creates a new performance optimizer.
func NewIoUringVsEpoll() *IoUringVsEpoll {
        return &IoUringVsEpoll{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *IoUringVsEpoll) Get(key uint64) (interface{, bool) {
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
func (p *IoUringVsEpoll) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *IoUringVsEpoll) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
