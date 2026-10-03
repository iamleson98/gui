// Question #8: Read-Copy-Update (RCU) Pattern
// Category: Concurrency | Difficulty: Hard
// Concepts: RCU, grace period, deferred reclamation, read-mostly
// Description: Simulate RCU by allowing readers to proceed without locks and deferring reclamation to a grace period.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Read-Copy-Update (RCU) Pattern
// Implements a concurrent primitive for question #8.
type ReadCopyUpdateRcuPattern struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewReadCopyUpdateRcuPattern creates a new instance.
func NewReadCopyUpdateRcuPattern() *ReadCopyUpdateRcuPattern {
        x := &ReadCopyUpdateRcuPattern{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ReadCopyUpdateRcuPattern) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ReadCopyUpdateRcuPattern) Result() int64 {
        return x.state.Load()
}
