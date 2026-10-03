// Question #28: Memory Barriers and Fences
// Category: Concurrency | Difficulty: Hard
// Concepts: memory fence, load-store, visibility, portability
// Description: Place read and write fences correctly so that lock-free algorithms publish visibility and consumption in the intended order.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Memory Barriers and Fences
// Implements a concurrent primitive for question #28.
type MemoryBarriersAndFences struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewMemoryBarriersAndFences creates a new instance.
func NewMemoryBarriersAndFences() *MemoryBarriersAndFences {
        x := &MemoryBarriersAndFences{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *MemoryBarriersAndFences) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *MemoryBarriersAndFences) Result() int64 {
        return x.state.Load()
}
