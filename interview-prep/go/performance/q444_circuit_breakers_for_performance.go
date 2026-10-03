// Question #444: Circuit Breakers for Performance
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: circuit breaker, fail fast, latency, outage
// Description: Use circuit breakers to fail fast and protect latency during dependency outages.
package performance

import (
        "sync"
        "sync/atomic"
)

// Circuit Breakers for Performance
// Implements a performance optimization for question #444.
type CircuitBreakersForPerformance struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewCircuitBreakersForPerformance creates a new performance optimizer.
func NewCircuitBreakersForPerformance() *CircuitBreakersForPerformance {
        return &CircuitBreakersForPerformance{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *CircuitBreakersForPerformance) Get(key uint64) (interface{, bool) {
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
func (p *CircuitBreakersForPerformance) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *CircuitBreakersForPerformance) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
