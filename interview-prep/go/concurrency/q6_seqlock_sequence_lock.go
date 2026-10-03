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
type SeqlockSequenceLock struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewSeqlockSequenceLock creates a new instance.
func NewSeqlockSequenceLock() *SeqlockSequenceLock {
        x := &SeqlockSequenceLock{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *SeqlockSequenceLock) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *SeqlockSequenceLock) Result() int64 {
        return x.state.Load()
}
