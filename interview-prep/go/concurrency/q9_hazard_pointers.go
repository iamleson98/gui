// Question #9: Hazard Pointers
// Category: Concurrency | Difficulty: Hard
// Concepts: hazard pointers, memory reclamation, ABA, lock-free
// Description: Implement hazard pointers so that a lock-free data structure safely defers reclamation of nodes a reader is inspecting.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Hazard Pointers
// Implements a concurrent primitive for question #9.
type Q9_HazardPointers struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ9_HazardPointers creates a new instance.
func NewQ9_HazardPointers() *Q9_HazardPointers {
        x := &Q9_HazardPointers{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q9_HazardPointers) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q9_HazardPointers) Result() int64 {
        return x.state.Load()
}
