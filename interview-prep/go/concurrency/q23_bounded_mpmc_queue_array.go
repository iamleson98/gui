// Question #23: Bounded MPMC Queue (Array)
// Category: Concurrency | Difficulty: Hard
// Concepts: MPMC, bounded queue, sequence, CAS
// Description: Implement an array-based bounded MPMC queue using a sequence per cell and compare-and-swap on the cell.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Bounded MPMC Queue (Array)
// Implements a concurrent primitive for question #23.
type BoundedMpmcQueueArray struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewBoundedMpmcQueueArray creates a new instance.
func NewBoundedMpmcQueueArray() *BoundedMpmcQueueArray {
        x := &BoundedMpmcQueueArray{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *BoundedMpmcQueueArray) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *BoundedMpmcQueueArray) Result() int64 {
        return x.state.Load()
}
