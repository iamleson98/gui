// Question #53: Futures and Promises
// Category: Concurrency | Difficulty: Hard
// Concepts: future, promise, continuation, shared state
// Description: Implement a future/promise pair with shared state, continuations, and ready/error/pending states.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Futures and Promises
// Implements a concurrent primitive for question #53.
type FuturesAndPromises struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewFuturesAndPromises creates a new instance.
func NewFuturesAndPromises() *FuturesAndPromises {
        x := &FuturesAndPromises{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *FuturesAndPromises) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *FuturesAndPromises) Result() int64 {
        return x.state.Load()
}
