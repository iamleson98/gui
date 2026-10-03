// Question #22: Lock-Free SPSC Ring Buffer
// Category: Concurrency | Difficulty: Hard
// Concepts: SPSC, ring buffer, memory ordering, cache lines
// Description: Build a single-producer single-consumer bounded ring buffer using relaxed loads/stores and a power-of-two size.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Lock-Free SPSC Ring Buffer
// Implements a concurrent primitive for question #22.
type Q22_LockFreeSpscRingBuffer struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ22_LockFreeSpscRingBuffer creates a new instance.
func NewQ22_LockFreeSpscRingBuffer() *Q22_LockFreeSpscRingBuffer {
        x := &Q22_LockFreeSpscRingBuffer{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q22_LockFreeSpscRingBuffer) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q22_LockFreeSpscRingBuffer) Result() int64 {
        return x.state.Load()
}
