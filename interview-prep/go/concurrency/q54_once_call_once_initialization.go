// Question #54: Once / Call-Once Initialization
// Category: Concurrency | Difficulty: Hard
// Concepts: once, initialization, double-checked, atomic
// Description: Implement std::once / sync.Once semantics guaranteeing a function runs exactly once under concurrency.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Once / Call-Once Initialization
// Implements a concurrent primitive for question #54.
type OnceCallOnceInitialization struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewOnceCallOnceInitialization creates a new instance.
func NewOnceCallOnceInitialization() *OnceCallOnceInitialization {
        x := &OnceCallOnceInitialization{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *OnceCallOnceInitialization) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *OnceCallOnceInitialization) Result() int64 {
        return x.state.Load()
}
