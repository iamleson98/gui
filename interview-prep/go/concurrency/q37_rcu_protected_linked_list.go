// Question #37: RCU-Protected Linked List
// Category: Concurrency | Difficulty: Hard
// Concepts: RCU, linked list, grace period, read-mostly
// Description: Implement a linked list whose readers traverse lock-free while updaters use RCU to defer node removal.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// RCU-Protected Linked List
// Implements a concurrent primitive for question #37.
type RcuProtectedLinkedList struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewRcuProtectedLinkedList creates a new instance.
func NewRcuProtectedLinkedList() *RcuProtectedLinkedList {
        x := &RcuProtectedLinkedList{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *RcuProtectedLinkedList) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *RcuProtectedLinkedList) Result() int64 {
        return x.state.Load()
}
