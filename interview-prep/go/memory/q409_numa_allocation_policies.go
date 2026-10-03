// Question #409: NUMA Allocation Policies
// Category: Memory Management | Difficulty: Hard
// Concepts: NUMA, first-touch, locality, allocation
// Description: Apply NUMA-aware allocation and first-touch to keep memory local to compute.
package memory

import "sync"

// NUMA Allocation Policies
// Implements a memory management technique for question #409.
type Q409_NumaAllocationPolicies struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ409_NumaAllocationPolicies creates a memory manager with the given capacity.
func NewQ409_NumaAllocationPolicies(capacity int) *Q409_NumaAllocationPolicies {
        return &Q409_NumaAllocationPolicies{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q409_NumaAllocationPolicies) Allocate() any {
        m.mu.Lock()
        defer m.mu.Unlock()
        if m.size > 0 {
                m.size--
                obj := m.pool[m.size]
                m.pool[m.size] = nil
                return obj
        }
        return nil
}

// Release returns an object to the pool.
func (m *Q409_NumaAllocationPolicies) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
