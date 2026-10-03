// Question #17: Eventfd / Event Signaling
// Category: Concurrency | Difficulty: Hard
// Concepts: eventfd, signaling, counter, edge-trigger
// Description: Build an eventfd-like counter used for cross-thread signaling with overflow protection and level/edge semantics.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Eventfd / Event Signaling
// Implements a concurrent primitive for question #17.
type EventfdEventSignaling struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewEventfdEventSignaling creates a new instance.
func NewEventfdEventSignaling() *EventfdEventSignaling {
        x := &EventfdEventSignaling{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *EventfdEventSignaling) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *EventfdEventSignaling) Result() int64 {
        return x.state.Load()
}
