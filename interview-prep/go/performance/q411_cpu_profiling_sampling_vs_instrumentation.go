// Question #411: CPU Profiling: Sampling vs Instrumentation
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: profiling, sampling, instrumentation, overhead
// Description: Contrast sampling and instrumentation profilers for accuracy vs overhead.
package performance

import (
        "sync"
        "sync/atomic"
)

// CPU Profiling: Sampling vs Instrumentation
// Implements a performance optimization for question #411.
type CpuProfilingSamplingVsInstrumentation struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewCpuProfilingSamplingVsInstrumentation creates a new performance optimizer.
func NewCpuProfilingSamplingVsInstrumentation() *CpuProfilingSamplingVsInstrumentation {
        return &CpuProfilingSamplingVsInstrumentation{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *CpuProfilingSamplingVsInstrumentation) Get(key uint64) (interface{, bool) {
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
func (p *CpuProfilingSamplingVsInstrumentation) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *CpuProfilingSamplingVsInstrumentation) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
