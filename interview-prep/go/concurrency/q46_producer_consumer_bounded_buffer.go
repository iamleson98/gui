// Question #46: Producer-Consumer Bounded Buffer
// Category: Concurrency | Difficulty: Hard
// Concepts: producer-consumer, bounded buffer, condition variable, backpressure
// Description: Implement the classic producer-consumer bounded buffer using a mutex and two condition variables.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Producer-Consumer Bounded Buffer
// Implements a concurrent primitive for question #46.
type Q46_ProducerConsumerBoundedBuffer struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewQ46_ProducerConsumerBoundedBuffer creates a new instance.
func NewQ46_ProducerConsumerBoundedBuffer() *Q46_ProducerConsumerBoundedBuffer {
        x := &Q46_ProducerConsumerBoundedBuffer{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *Q46_ProducerConsumerBoundedBuffer) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *Q46_ProducerConsumerBoundedBuffer) Result() int64 {
        return x.state.Load()
}
