// Question #27: Memory Ordering: Acquire/Release vs seq_cst
// Category: Concurrency | Difficulty: Hard
// Concepts: memory ordering, acquire/release, seq_cst, reordered
// Description: Compare acquire/release, relaxed, and sequentially-consistent ordering and pick the weakest safe ordering per access.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Memory Ordering: Acquire/Release vs seq_cst
// Implements a concurrent primitive for question #27.
type Q27_MemoryOrderingAcquireReleaseVsSeqCst struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ27_MemoryOrderingAcquireReleaseVsSeqCst creates a new instance.
func NewQ27_MemoryOrderingAcquireReleaseVsSeqCst() *Q27_MemoryOrderingAcquireReleaseVsSeqCst {
        x := &Q27_MemoryOrderingAcquireReleaseVsSeqCst{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q27_MemoryOrderingAcquireReleaseVsSeqCst) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q27_MemoryOrderingAcquireReleaseVsSeqCst) Result() int64 {
        return x.state.Load()
}
