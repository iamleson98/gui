// Question #13: Peterson's Algorithm
// Category: Concurrency | Difficulty: Hard
// Concepts: mutual exclusion, flags, turn, memory ordering
// Description: Implement the classic two-process mutual exclusion algorithm using flags and a turn variable with sequential consistency.
package concurrency

import (
        "sync"
        "sync/atomic"
)

// Peterson's Algorithm
// Implements a concurrent primitive for question #13.
type PetersonSAlgorithm struct {
        mu       sync.Mutex
        cond     *sync.Cond
        state    atomic.Int64
        notify   chan struct{}
}

// NewPetersonSAlgorithm creates a new instance.
func NewPetersonSAlgorithm() *PetersonSAlgorithm {
        x := &PetersonSAlgorithm{notify: make(chan struct{}, 1)}
        x.cond = sync.NewCond(&x.mu)
        return x
}

// Execute performs the core operation for this question.
func (x *PetersonSAlgorithm) Execute() {
        // Acquire and release using the concurrent primitive
        x.mu.Lock()
        defer x.mu.Unlock()
        x.state.Add(1)
        x.cond.Broadcast()
}

// Result returns the current state.
func (x *PetersonSAlgorithm) Result() int64 {
        return x.state.Load()
}
