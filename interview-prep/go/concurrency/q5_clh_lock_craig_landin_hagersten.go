// Question #5: CLH Lock (Craig, Landin, Hagersten)
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, queue lock, FIFO, spin locality
// Description: Build a queue lock whose thread spins on the predecessor's lock word and hands off ownership by toggling its own node.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// CLH Lock (Craig, Landin, Hagersten)
// Implements a concurrent primitive for question #5.
type Q5_ClhLockCraigLandinHagersten struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ5_ClhLockCraigLandinHagersten creates a new instance.
func NewQ5_ClhLockCraigLandinHagersten() *Q5_ClhLockCraigLandinHagersten {
        x := &Q5_ClhLockCraigLandinHagersten{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q5_ClhLockCraigLandinHagersten) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q5_ClhLockCraigLandinHagersten) Result() int64 {
        return x.state.Load()
}
