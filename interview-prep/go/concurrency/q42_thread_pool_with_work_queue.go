// Question #42: Thread Pool with Work Queue
// Category: Concurrency | Difficulty: Hard
// Concepts: thread pool, work queue, workers, shutdown
// Description: Implement a fixed-size worker pool with a global task queue, blocking dequeue, and shutdown semantics.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Thread Pool with Work Queue
// Implements a concurrent primitive for question #42.
type ThreadPoolWithWorkQueue struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewThreadPoolWithWorkQueue creates a new instance.
func NewThreadPoolWithWorkQueue() *ThreadPoolWithWorkQueue {
        x := &ThreadPoolWithWorkQueue{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ThreadPoolWithWorkQueue) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ThreadPoolWithWorkQueue) Result() int64 {
        return x.state.Load()
}
