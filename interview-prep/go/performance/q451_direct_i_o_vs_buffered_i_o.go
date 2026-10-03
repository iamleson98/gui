// Question #451: Direct I/O vs Buffered I/O
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: direct I/O, buffered I/O, page cache, O_DIRECT
// Description: Contrast O_DIRECT with buffered I/O and the page-cache implications.
package performance

import (
        "sync"
        "sync/atomic"
)

// Direct I/O vs Buffered I/O
// Implements a performance optimization for question #451.
type DirectIOVsBufferedIO struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewDirectIOVsBufferedIO creates a new performance optimizer.
func NewDirectIOVsBufferedIO() *DirectIOVsBufferedIO {
        return &DirectIOVsBufferedIO{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *DirectIOVsBufferedIO) Get(key uint64) (interface{, bool) {
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
func (p *DirectIOVsBufferedIO) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *DirectIOVsBufferedIO) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
