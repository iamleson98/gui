// Question #51: CountDownLatch Equivalent
// Category: Concurrency | Difficulty: Hard
// Concepts: latch, one-shot, counter, park
// Description: Implement a one-shot latch that blocks threads until a counter reaches zero.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// CountDownLatch Equivalent
// Implements a concurrent primitive for question #51.
type CountdownlatchEquivalent struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewCountdownlatchEquivalent creates a new instance.
func NewCountdownlatchEquivalent() *CountdownlatchEquivalent {
        x := &CountdownlatchEquivalent{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *CountdownlatchEquivalent) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *CountdownlatchEquivalent) Result() int64 {
        return x.state.Load()
}
