// Question #52: Q52_Exchanger
// Category: Concurrency | Difficulty: Hard
// Concepts: exchanger, rendezvous, swap, two threads
// Description: Build an exchanger where two threads rendezvous and swap values atomically.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Q52_Exchanger
// Implements a concurrent primitive for question #52.
type Q52_Exchanger struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ52_Exchanger creates a new instance.
func NewQ52_Exchanger() *Q52_Exchanger {
        x := &Q52_Exchanger{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q52_Exchanger) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q52_Exchanger) Result() int64 {
        return x.state.Load()
}
