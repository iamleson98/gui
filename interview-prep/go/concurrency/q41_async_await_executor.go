// Question #41: Async/Await Executor
// Category: Concurrency | Difficulty: Hard
// Concepts: async/await, executor, waker, poll
// Description: Build a single-threaded cooperative task executor with a ready queue, polling, and wakers for async/await.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Async/Await Executor
// Implements a concurrent primitive for question #41.
type Q41_AsyncAwaitExecutor struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ41_AsyncAwaitExecutor creates a new instance.
func NewQ41_AsyncAwaitExecutor() *Q41_AsyncAwaitExecutor {
        x := &Q41_AsyncAwaitExecutor{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q41_AsyncAwaitExecutor) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q41_AsyncAwaitExecutor) Result() int64 {
        return x.state.Load()
}
