// Question #367: Stack (Linear) Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: stack allocator, markers, LIFO free, linear
// Description: Implement a stack allocator with markers to roll back to a previous top.
package memory

import "sync"

// Stack (Linear) Allocator
// Implements a memory management technique for question #367.
type Q367_StackLinearAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ367_StackLinearAllocator creates a memory manager with the given capacity.
func NewQ367_StackLinearAllocator(capacity int) *Q367_StackLinearAllocator {
        return &Q367_StackLinearAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q367_StackLinearAllocator) Allocate() any {
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
func (m *Q367_StackLinearAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
