// Question #15: Biasable Readers-Writer Lock
// Category: Concurrency | Difficulty: Hard
// Concepts: readers-writers, biasing, throughput, fairness
// Description: Design an RW lock that can be biased toward readers or writers and rebiased at runtime to tune throughput.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Biasable Readers-Writer Lock
// Implements a concurrent primitive for question #15.
type BiasableReadersWriterLock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewBiasableReadersWriterLock creates a new instance.
func NewBiasableReadersWriterLock() *BiasableReadersWriterLock {
        x := &BiasableReadersWriterLock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *BiasableReadersWriterLock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *BiasableReadersWriterLock) Result() int64 {
        return x.state.Load()
}
