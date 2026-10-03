// Question #36: Split-Ordered List
// Category: Concurrency | Difficulty: Hard
// Concepts: split-ordered list, lock-free, sorted list, hash
// Description: Implement a lock-free hash table based on a sorted linked list with reverse-key ordering (Shalev-Shavit).
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Split-Ordered List
// Implements a concurrent primitive for question #36.
type Q36_SplitOrderedList struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ36_SplitOrderedList creates a new instance.
func NewQ36_SplitOrderedList() *Q36_SplitOrderedList {
        x := &Q36_SplitOrderedList{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q36_SplitOrderedList) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q36_SplitOrderedList) Result() int64 {
        return x.state.Load()
}
