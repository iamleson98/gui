// Question #55: Double-Checked Locking
// Category: Concurrency | Difficulty: Hard
// Concepts: double-checked locking, singleton, memory ordering, fence
// Description: Implement a correct double-checked locking singleton using acquire/release fences to avoid the classic pitfall.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Double-Checked Locking
// Implements a concurrent primitive for question #55.
type Q55_DoubleCheckedLocking struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ55_DoubleCheckedLocking creates a new instance.
func NewQ55_DoubleCheckedLocking() *Q55_DoubleCheckedLocking {
        x := &Q55_DoubleCheckedLocking{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q55_DoubleCheckedLocking) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q55_DoubleCheckedLocking) Result() int64 {
        return x.state.Load()
}
