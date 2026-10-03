// Question #33: LMAX Disruptor Pattern
// Category: Concurrency | Difficulty: Hard
// Concepts: Disruptor, ring, sequences, batching
// Description: Build a Disruptor-style ring with sequenced consumers, gating sequences, and a batched publisher.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// LMAX Disruptor Pattern
// Implements a concurrent primitive for question #33.
type Q33_LmaxDisruptorPattern struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ33_LmaxDisruptorPattern creates a new instance.
func NewQ33_LmaxDisruptorPattern() *Q33_LmaxDisruptorPattern {
        x := &Q33_LmaxDisruptorPattern{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q33_LmaxDisruptorPattern) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q33_LmaxDisruptorPattern) Result() int64 {
        return x.state.Load()
}
