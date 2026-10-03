// Question #50: Phaser / Cyclic Barrier
// Category: Concurrency | Difficulty: Hard
// Concepts: phaser, cyclic barrier, parties, phases
// Description: Design a phaser supporting dynamic party registration, arrivals, and phase advancement.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Phaser / Cyclic Barrier
// Implements a concurrent primitive for question #50.
type Q50_PhaserCyclicBarrier struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ50_PhaserCyclicBarrier creates a new instance.
func NewQ50_PhaserCyclicBarrier() *Q50_PhaserCyclicBarrier {
        x := &Q50_PhaserCyclicBarrier{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q50_PhaserCyclicBarrier) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q50_PhaserCyclicBarrier) Result() int64 {
        return x.state.Load()
}
