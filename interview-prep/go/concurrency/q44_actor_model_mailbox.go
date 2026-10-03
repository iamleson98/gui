// Question #44: Actor Model Mailbox
// Category: Concurrency | Difficulty: Hard
// Concepts: actor model, mailbox, message passing, dispatcher
// Description: Implement an actor runtime with per-actor mailboxes, message ordering, and a dispatcher.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Actor Model Mailbox
// Implements a concurrent primitive for question #44.
type Q44_ActorModelMailbox struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ44_ActorModelMailbox creates a new instance.
func NewQ44_ActorModelMailbox() *Q44_ActorModelMailbox {
        x := &Q44_ActorModelMailbox{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q44_ActorModelMailbox) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q44_ActorModelMailbox) Result() int64 {
        return x.state.Load()
}
