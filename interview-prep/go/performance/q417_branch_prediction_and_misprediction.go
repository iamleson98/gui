// Question #417: Branch Prediction and Misprediction
// Category: Performance & Profiling | Difficulty: Hard
// Concepts: branch prediction, misprediction, pipeline, branchless
// Description: Reason about branch prediction cost and restructure code to be branch-predictable.
package performance

import (
        "sync"
        "sync/atomic"
)

// Branch Prediction and Misprediction
// Implements a performance optimization for question #417.
type Q417_BranchPredictionAndMisprediction struct {
        mu      sync.Mutex
        cache   map[uint64]any
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewQ417_BranchPredictionAndMisprediction creates a new performance optimizer.
func NewQ417_BranchPredictionAndMisprediction() *Q417_BranchPredictionAndMisprediction {
        return &Q417_BranchPredictionAndMisprediction{cache: make(map[uint64]any)}
}

// Get retrieves a cached value or returns false.
func (p *Q417_BranchPredictionAndMisprediction) Get(key uint64) (any, bool) {
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
func (p *Q417_BranchPredictionAndMisprediction) Set(key uint64, val any) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *Q417_BranchPredictionAndMisprediction) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
