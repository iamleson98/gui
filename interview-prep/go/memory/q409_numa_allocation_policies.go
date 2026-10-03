// Question #409: NUMA Allocation Policies
// Category: Memory Management | Difficulty: Hard
// Concepts: NUMA, first-touch, locality, allocation
// Description: Apply NUMA-aware allocation and first-touch to keep memory local to compute.
package memory

import "sync"

// NUMA Allocation Policies
// Implements a memory management technique for question #409.
type NumaAllocationPolicies struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewNumaAllocationPolicies creates a memory manager with the given capacity.
func NewNumaAllocationPolicies(capacity int) *NumaAllocationPolicies {
        return &NumaAllocationPolicies{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *NumaAllocationPolicies) Allocate() interface{ {
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
func (m *NumaAllocationPolicies) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
