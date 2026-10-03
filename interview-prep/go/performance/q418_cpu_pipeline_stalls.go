// Question #418: CPU Pipeline Stalls
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: pipeline, stall, data hazard, latency
// Description: Identify pipeline stalls from data hazards and long-latency instructions.
package performance

import (
        "sync"
        "sync/atomic"
)

// CPU Pipeline Stalls
// Implements a performance optimization for question #418.
type CpuPipelineStalls struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewCpuPipelineStalls creates a new performance optimizer.
func NewCpuPipelineStalls() *CpuPipelineStalls {
        return &CpuPipelineStalls{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *CpuPipelineStalls) Get(key uint64) (interface{, bool) {
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
func (p *CpuPipelineStalls) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *CpuPipelineStalls) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
