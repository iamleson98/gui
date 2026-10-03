// Question #58: Exponential Backoff Strategies
// Category: Concurrency | Difficulty: Hard
// Concepts: backoff, exponential, jitter, contention
// Description: Build an exponential backoff with jitter for retrying contended CAS loops and RPC calls.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Exponential Backoff Strategies
// Implements a concurrent primitive for question #58.
type Q58_ExponentialBackoffStrategies struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ58_ExponentialBackoffStrategies creates a new instance.
func NewQ58_ExponentialBackoffStrategies() *Q58_ExponentialBackoffStrategies {
        x := &Q58_ExponentialBackoffStrategies{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q58_ExponentialBackoffStrategies) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q58_ExponentialBackoffStrategies) Result() int64 {
        return x.state.Load()
}
