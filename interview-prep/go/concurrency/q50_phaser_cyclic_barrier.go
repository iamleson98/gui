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
type PhaserCyclicBarrier struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewPhaserCyclicBarrier creates a new instance.
func NewPhaserCyclicBarrier() *PhaserCyclicBarrier {
        x := &PhaserCyclicBarrier{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *PhaserCyclicBarrier) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *PhaserCyclicBarrier) Result() int64 {
        return x.state.Load()
}
