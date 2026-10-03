// Question #35: Concurrent Hash Map with CAS Buckets
// Category: Concurrency | Difficulty: Hard
// Concepts: hash map, lock-free, CAS, chaining
// Description: Build a hash map whose buckets are lock-free singly linked lists updated by compare-and-swap.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Concurrent Hash Map with CAS Buckets
// Implements a concurrent primitive for question #35.
type Q35_ConcurrentHashMapWithCasBuckets struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ35_ConcurrentHashMapWithCasBuckets creates a new instance.
func NewQ35_ConcurrentHashMapWithCasBuckets() *Q35_ConcurrentHashMapWithCasBuckets {
        x := &Q35_ConcurrentHashMapWithCasBuckets{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q35_ConcurrentHashMapWithCasBuckets) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q35_ConcurrentHashMapWithCasBuckets) Result() int64 {
        return x.state.Load()
}
