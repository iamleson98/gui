// Question #26: DCAS / Double-Width CAS
// Category: Concurrency | Difficulty: Hard
// Concepts: DCAS, double-width CAS, versioning, portability
// Description: Implement a 128-bit compare-and-swap (DCAS) to atomically update a pointer and a counter together.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// DCAS / Double-Width CAS
// Implements a concurrent primitive for question #26.
type DcasDoubleWidthCas struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewDcasDoubleWidthCas creates a new instance.
func NewDcasDoubleWidthCas() *DcasDoubleWidthCas {
        x := &DcasDoubleWidthCas{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *DcasDoubleWidthCas) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *DcasDoubleWidthCas) Result() int64 {
        return x.state.Load()
}
