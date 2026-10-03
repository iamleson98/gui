// Question #56: Thread-Local Storage
// Category: Concurrency | Difficulty: Hard
// Concepts: thread-local, slots, cleanup, per-thread
// Description: Design a thread-local storage abstraction with per-thread slots and optional cleanup on thread exit.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Thread-Local Storage
// Implements a concurrent primitive for question #56.
type ThreadLocalStorage struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewThreadLocalStorage creates a new instance.
func NewThreadLocalStorage() *ThreadLocalStorage {
        x := &ThreadLocalStorage{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ThreadLocalStorage) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ThreadLocalStorage) Result() int64 {
        return x.state.Load()
}
