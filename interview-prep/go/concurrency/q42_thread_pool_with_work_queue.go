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
type Q42_ThreadPoolWithWorkQueue struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ42_ThreadPoolWithWorkQueue creates a new instance.
func NewQ42_ThreadPoolWithWorkQueue() *Q42_ThreadPoolWithWorkQueue {
        x := &Q42_ThreadPoolWithWorkQueue{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q42_ThreadPoolWithWorkQueue) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q42_ThreadPoolWithWorkQueue) Result() int64 {
        return x.state.Load()
}
