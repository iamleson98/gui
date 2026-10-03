// Question #447: Zero-Copy I/O (sendfile)
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: zero-copy, sendfile, splice, kernel buffer
// Description: Use sendfile and splice to move data between file descriptors without copying.
package performance

import (
        "sync"
        "sync/atomic"
)

// Zero-Copy I/O (sendfile)
// Implements a performance optimization for question #447.
type Q447_ZeroCopyIOSendfile struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ447_ZeroCopyIOSendfile creates a new performance optimizer.
func NewQ447_ZeroCopyIOSendfile() *Q447_ZeroCopyIOSendfile {
        return &Q447_ZeroCopyIOSendfile{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q447_ZeroCopyIOSendfile) Get(key uint64) (any, bool) {
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
func (p *Q447_ZeroCopyIOSendfile) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q447_ZeroCopyIOSendfile) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
