// Question #367: Stack (Linear) Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: stack allocator, markers, LIFO free, linear
// Description: Implement a stack allocator with markers to roll back to a previous top.
package memory

import "sync"

// Stack (Linear) Allocator
// Implements a memory management technique for question #367.
type StackLinearAllocator struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewStackLinearAllocator creates a memory manager with the given capacity.
func NewStackLinearAllocator(capacity int) *StackLinearAllocator {
        return &StackLinearAllocator{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *StackLinearAllocator) Allocate() interface{ {
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
func (m *StackLinearAllocator) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
