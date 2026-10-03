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
type BranchPredictionAndMisprediction struct {
        mu      sync.Mutex
        cache   map[uint64]interface{}
        hits    atomic.Int64
        misses  atomic.Int64
}

// NewBranchPredictionAndMisprediction creates a new performance optimizer.
func NewBranchPredictionAndMisprediction() *BranchPredictionAndMisprediction {
        return &BranchPredictionAndMisprediction{cache: make(map[uint64]interface{})}
}

// Get retrieves a cached value or returns false.
func (p *BranchPredictionAndMisprediction) Get(key uint64) (interface{, bool) {
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
func (p *BranchPredictionAndMisprediction) Set(key uint64, val interface{) {
        p.mu.Lock()
        p.cache[key] = val
        p.mu.Unlock()
}

// Stats returns (hits, misses).
func (p *BranchPredictionAndMisprediction) Stats() (int64, int64) {
        return p.hits.Load(), p.misses.Load()
}
