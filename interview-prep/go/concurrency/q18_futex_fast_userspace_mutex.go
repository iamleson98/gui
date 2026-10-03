// Question #18: Futex (Fast Userspace Mutex)
// Category: Concurrency | Difficulty: Hard
// Concepts: futex, mutex, kernel parking, wait queue
// Description: Implement a userspace mutex that spins on an atomic word and parks in the kernel only on contention.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Futex (Fast Userspace Mutex)
// Implements a concurrent primitive for question #18.
type Q18_FutexFastUserspaceMutex struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ18_FutexFastUserspaceMutex creates a new instance.
func NewQ18_FutexFastUserspaceMutex() *Q18_FutexFastUserspaceMutex {
        x := &Q18_FutexFastUserspaceMutex{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q18_FutexFastUserspaceMutex) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q18_FutexFastUserspaceMutex) Result() int64 {
        return x.state.Load()
}
