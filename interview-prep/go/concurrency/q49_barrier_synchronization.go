// Question #49: Barrier Synchronization
// Category: Concurrency | Difficulty: Hard
// Concepts: barrier, sense reversal, reuse, wait
// Description: Implement a reusable barrier where N threads wait and then all proceed, with sense reversal.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Barrier Synchronization
// Implements a concurrent primitive for question #49.
type BarrierSynchronization struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewBarrierSynchronization creates a new instance.
func NewBarrierSynchronization() *BarrierSynchronization {
        x := &BarrierSynchronization{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *BarrierSynchronization) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *BarrierSynchronization) Result() int64 {
        return x.state.Load()
}
