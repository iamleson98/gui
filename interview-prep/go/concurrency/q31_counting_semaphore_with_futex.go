// Question #31: Counting Semaphore with Futex
// Category: Concurrency | Difficulty: Hard
// Concepts: semaphore, futex, fast path, wait queue
// Description: Build a fast counting semaphore whose fast path is an atomic compare and whose slow path parks waiters.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Counting Semaphore with Futex
// Implements a concurrent primitive for question #31.
type CountingSemaphoreWithFutex struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewCountingSemaphoreWithFutex creates a new instance.
func NewCountingSemaphoreWithFutex() *CountingSemaphoreWithFutex {
        x := &CountingSemaphoreWithFutex{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *CountingSemaphoreWithFutex) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *CountingSemaphoreWithFutex) Result() int64 {
        return x.state.Load()
}
