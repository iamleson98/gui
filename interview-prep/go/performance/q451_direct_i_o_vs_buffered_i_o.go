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
type Q451_DirectIOVsBufferedIO struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ451_DirectIOVsBufferedIO creates a new performance optimizer.
func NewQ451_DirectIOVsBufferedIO() *Q451_DirectIOVsBufferedIO {
        return &Q451_DirectIOVsBufferedIO{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q451_DirectIOVsBufferedIO) Get(key uint64) (any, bool) {
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
func (p *Q451_DirectIOVsBufferedIO) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q451_DirectIOVsBufferedIO) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
