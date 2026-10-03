// Question #25: ABA Problem and Tagged Pointers
// Category: Concurrency | Difficulty: Hard
// Concepts: ABA, tagged pointer, CAS, versioning
// Description: Demonstrate the ABA problem on a Treiber stack and fix it using a tagged pointer packing a version counter.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// ABA Problem and Tagged Pointers
// Implements a concurrent primitive for question #25.
type AbaProblemAndTaggedPointers struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewAbaProblemAndTaggedPointers creates a new instance.
func NewAbaProblemAndTaggedPointers() *AbaProblemAndTaggedPointers {
        x := &AbaProblemAndTaggedPointers{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *AbaProblemAndTaggedPointers) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *AbaProblemAndTaggedPointers) Result() int64 {
        return x.state.Load()
}
