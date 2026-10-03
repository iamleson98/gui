// Question #60: Coherence Traffic and False Sharing
// Category: Concurrency | Difficulty: Hard
// Concepts: false sharing, cache line, padding, coherence
// Description: Diagnose false sharing between adjacent atomics on the same cache line and pad to eliminate coherence traffic.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Coherence Traffic and False Sharing
// Implements a concurrent primitive for question #60.
type CoherenceTrafficAndFalseSharing struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewCoherenceTrafficAndFalseSharing creates a new instance.
func NewCoherenceTrafficAndFalseSharing() *CoherenceTrafficAndFalseSharing {
        x := &CoherenceTrafficAndFalseSharing{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *CoherenceTrafficAndFalseSharing) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *CoherenceTrafficAndFalseSharing) Result() int64 {
        return x.state.Load()
}
