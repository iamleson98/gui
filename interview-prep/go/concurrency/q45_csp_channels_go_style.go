// Question #45: CSP Channels (Go-style)
// Category: Concurrency | Difficulty: Hard
// Concepts: channels, CSP, select, rendezvous
// Description: Build unbuffered and buffered channels with select, close, and fair rendezvous semantics.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// CSP Channels (Go-style)
// Implements a concurrent primitive for question #45.
type CspChannelsGoStyle struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewCspChannelsGoStyle creates a new instance.
func NewCspChannelsGoStyle() *CspChannelsGoStyle {
        x := &CspChannelsGoStyle{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *CspChannelsGoStyle) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *CspChannelsGoStyle) Result() int64 {
        return x.state.Load()
}
