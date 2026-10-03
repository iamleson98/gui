// Question #57: Async Mutex / Async-Aware Lock
// Category: Concurrency | Difficulty: Hard
// Concepts: async, mutex, wait list, parking
// Description: Implement a mutex whose waiters park futures rather than OS threads, avoiding thread blocking.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Async Mutex / Async-Aware Lock
// Implements a concurrent primitive for question #57.
type Q57_AsyncMutexAsyncAwareLock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ57_AsyncMutexAsyncAwareLock creates a new instance.
func NewQ57_AsyncMutexAsyncAwareLock() *Q57_AsyncMutexAsyncAwareLock {
        x := &Q57_AsyncMutexAsyncAwareLock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q57_AsyncMutexAsyncAwareLock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q57_AsyncMutexAsyncAwareLock) Result() int64 {
        return x.state.Load()
}
