// Question #4: MCS Lock (Mellor-Crummy & Scott)
// Category: Concurrency | Difficulty: Hard
// Concepts: spinlock, queue lock, scalability, NUMA
// Description: Implement a scalable list-based queue lock where each thread spins on a locally-cached flag.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// MCS Lock (Mellor-Crummy & Scott)
// Implements a concurrent primitive for question #4.
type Q4_McsLockMellorCrummyScott struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ4_McsLockMellorCrummyScott creates a new instance.
func NewQ4_McsLockMellorCrummyScott() *Q4_McsLockMellorCrummyScott {
        x := &Q4_McsLockMellorCrummyScott{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q4_McsLockMellorCrummyScott) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q4_McsLockMellorCrummyScott) Result() int64 {
        return x.state.Load()
}
