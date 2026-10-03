// Question #7: Adaptive Spinlock
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, futex, backoff, hybrid
// Description: Design a spinlock that spins briefly then falls back to a kernel futex or parking primitive to avoid wasted CPU.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Adaptive Spinlock
// Implements a concurrent primitive for question #7.
type Q7_AdaptiveSpinlock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ7_AdaptiveSpinlock creates a new instance.
func NewQ7_AdaptiveSpinlock() *Q7_AdaptiveSpinlock {
        x := &Q7_AdaptiveSpinlock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q7_AdaptiveSpinlock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q7_AdaptiveSpinlock) Result() int64 {
        return x.state.Load()
}
