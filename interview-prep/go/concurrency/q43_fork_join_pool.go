// Question #43: Fork-Join Pool
// Category: Concurrency | Difficulty: Hard
// Concepts: fork-join, work stealing, recursive, divide and conquer
// Description: Design a fork-join executor with work-stealing deques, barrier joins, and recursive task splitting.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Fork-Join Pool
// Implements a concurrent primitive for question #43.
type ForkJoinPool struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewForkJoinPool creates a new instance.
func NewForkJoinPool() *ForkJoinPool {
        x := &ForkJoinPool{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ForkJoinPool) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ForkJoinPool) Result() int64 {
        return x.state.Load()
}
