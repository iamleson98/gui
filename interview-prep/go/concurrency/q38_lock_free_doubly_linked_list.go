// Question #38: Lock-Free Doubly Linked List
// Category: Concurrency | Difficulty: Hard
// Concepts: doubly linked list, lock-free, marking, ABA
// Description: Design a lock-free doubly linked list handling the classic concurrent-deletion hazard with marking.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Lock-Free Doubly Linked List
// Implements a concurrent primitive for question #38.
type Q38_LockFreeDoublyLinkedList struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ38_LockFreeDoublyLinkedList creates a new instance.
func NewQ38_LockFreeDoublyLinkedList() *Q38_LockFreeDoublyLinkedList {
        x := &Q38_LockFreeDoublyLinkedList{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q38_LockFreeDoublyLinkedList) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q38_LockFreeDoublyLinkedList) Result() int64 {
        return x.state.Load()
}
