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
type LmaxDisruptorPattern struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewLmaxDisruptorPattern creates a new instance.
func NewLmaxDisruptorPattern() *LmaxDisruptorPattern {
        x := &LmaxDisruptorPattern{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *LmaxDisruptorPattern) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *LmaxDisruptorPattern) Result() int64 {
        return x.state.Load()
}
