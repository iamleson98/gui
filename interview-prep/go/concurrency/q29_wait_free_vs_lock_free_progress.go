// Question #29: Wait-Free vs Lock-Free Progress
// Category: Concurrency | Difficulty: Hard
// Concepts: wait-free, lock-free, progress, bounded steps
// Description: Design a wait-free queue where every operation completes in a bounded number of steps, and contrast with lock-free progress.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Wait-Free vs Lock-Free Progress
// Implements a concurrent primitive for question #29.
type WaitFreeVsLockFreeProgress struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewWaitFreeVsLockFreeProgress creates a new instance.
func NewWaitFreeVsLockFreeProgress() *WaitFreeVsLockFreeProgress {
        x := &WaitFreeVsLockFreeProgress{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *WaitFreeVsLockFreeProgress) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *WaitFreeVsLockFreeProgress) Result() int64 {
        return x.state.Load()
}
