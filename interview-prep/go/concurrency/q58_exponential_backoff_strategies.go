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
type ExponentialBackoffStrategies struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewExponentialBackoffStrategies creates a new instance.
func NewExponentialBackoffStrategies() *ExponentialBackoffStrategies {
        x := &ExponentialBackoffStrategies{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ExponentialBackoffStrategies) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ExponentialBackoffStrategies) Result() int64 {
        return x.state.Load()
}
