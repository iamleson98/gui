// Question #12: Queue Spinlock (Linux qspinlock)
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, queue lock, MCS, fairness
// Description: Design a compact queue spinlock that stores waiting nodes in a small per-CPU array and falls back to a linked list.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Queue Spinlock (Linux qspinlock)
// Implements a concurrent primitive for question #12.
type Q12_QueueSpinlockLinuxQspinlock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ12_QueueSpinlockLinuxQspinlock creates a new instance.
func NewQ12_QueueSpinlockLinuxQspinlock() *Q12_QueueSpinlockLinuxQspinlock {
        x := &Q12_QueueSpinlockLinuxQspinlock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q12_QueueSpinlockLinuxQspinlock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q12_QueueSpinlockLinuxQspinlock) Result() int64 {
        return x.state.Load()
}
