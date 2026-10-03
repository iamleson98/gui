// Question #6: SeqLock (Sequence Lock)
// Category: Concurrency | Difficulty: Hard
// Concepts: seqlock, readers-writers, memory ordering, fences
// Description: Implement a sequence-lock reader/writer pattern allowing lock-free reads while writes increment a counter twice.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// SeqLock (Sequence Lock)
// Implements a concurrent primitive for question #6.
type Q6_SeqlockSequenceLock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ6_SeqlockSequenceLock creates a new instance.
func NewQ6_SeqlockSequenceLock() *Q6_SeqlockSequenceLock {
        x := &Q6_SeqlockSequenceLock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q6_SeqlockSequenceLock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q6_SeqlockSequenceLock) Result() int64 {
        return x.state.Load()
}
