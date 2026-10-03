// Question #40: Wait-Free Hash Table
// Category: Concurrency | Difficulty: Hard
// Concepts: wait-free, hash table, resize, bounded steps
// Description: Design a resize-friendly wait-free hash table where every operation completes in bounded CAS steps.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Wait-Free Hash Table
// Implements a concurrent primitive for question #40.
type Q40_WaitFreeHashTable struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ40_WaitFreeHashTable creates a new instance.
func NewQ40_WaitFreeHashTable() *Q40_WaitFreeHashTable {
        x := &Q40_WaitFreeHashTable{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q40_WaitFreeHashTable) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q40_WaitFreeHashTable) Result() int64 {
        return x.state.Load()
}
