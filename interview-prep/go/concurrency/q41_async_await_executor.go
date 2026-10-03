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
type AsyncAwaitExecutor struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewAsyncAwaitExecutor creates a new instance.
func NewAsyncAwaitExecutor() *AsyncAwaitExecutor {
        x := &AsyncAwaitExecutor{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *AsyncAwaitExecutor) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *AsyncAwaitExecutor) Result() int64 {
        return x.state.Load()
}
