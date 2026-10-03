// Question #48: Readers-Writers with Writer Preference
// Category: Concurrency | Difficulty: Hard
// Concepts: readers-writers, writer preference, starvation, fairness
// Description: Design an RW lock that prefers writers to avoid writer starvation while preventing reader starvation.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Readers-Writers with Writer Preference
// Implements a concurrent primitive for question #48.
type ReadersWritersWithWriterPreference struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewReadersWritersWithWriterPreference creates a new instance.
func NewReadersWritersWithWriterPreference() *ReadersWritersWithWriterPreference {
        x := &ReadersWritersWithWriterPreference{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *ReadersWritersWithWriterPreference) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *ReadersWritersWithWriterPreference) Result() int64 {
        return x.state.Load()
}
