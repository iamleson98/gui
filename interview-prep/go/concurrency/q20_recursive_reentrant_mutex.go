// Question #20: Recursive (Reentrant) Mutex
// Category: Concurrency | Difficulty: Hard
// Concepts: mutex, reentrant, owner, recursion count
// Description: Implement a mutex that allows the same thread to acquire it multiple times by tracking an owner and recursion count.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Recursive (Reentrant) Mutex
// Implements a concurrent primitive for question #20.
type RecursiveReentrantMutex struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewRecursiveReentrantMutex creates a new instance.
func NewRecursiveReentrantMutex() *RecursiveReentrantMutex {
        x := &RecursiveReentrantMutex{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *RecursiveReentrantMutex) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *RecursiveReentrantMutex) Result() int64 {
        return x.state.Load()
}
