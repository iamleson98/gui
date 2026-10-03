// Question #59: NUMA-Aware Synchronization
// Category: Concurrency | Difficulty: Hard
// Concepts: NUMA, topology, data placement, scalability
// Description: Design locks and data placement that respect NUMA topology to reduce cross-socket traffic.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// NUMA-Aware Synchronization
// Implements a concurrent primitive for question #59.
type NumaAwareSynchronization struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewNumaAwareSynchronization creates a new instance.
func NewNumaAwareSynchronization() *NumaAwareSynchronization {
        x := &NumaAwareSynchronization{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *NumaAwareSynchronization) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *NumaAwareSynchronization) Result() int64 {
        return x.state.Load()
}
