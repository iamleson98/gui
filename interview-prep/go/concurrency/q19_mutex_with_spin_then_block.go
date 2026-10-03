// Question #19: Mutex with Spin-then-Block
// Category: Concurrency | Difficulty: Hard
// Concepts: mutex, spin-then-block, futex, latency
// Description: Design a mutex that spins briefly in userspace and only then issues a system call to park the thread.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Mutex with Spin-then-Block
// Implements a concurrent primitive for question #19.
type MutexWithSpinThenBlock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewMutexWithSpinThenBlock creates a new instance.
func NewMutexWithSpinThenBlock() *MutexWithSpinThenBlock {
        x := &MutexWithSpinThenBlock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *MutexWithSpinThenBlock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *MutexWithSpinThenBlock) Result() int64 {
        return x.state.Load()
}
