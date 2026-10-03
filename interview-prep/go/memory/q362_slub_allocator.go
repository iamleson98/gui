// Question #362: Slub Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: SLUB, per-CPU, freelist, kernel
// Description: Explain the SLUB allocator's simpler, per-CPU design replacing the classic slab allocator.
package memory

import "sync"

// Slub Allocator
// Implements a memory management technique for question #362.
type SlubAllocator struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewSlubAllocator creates a memory manager with the given capacity.
func NewSlubAllocator(capacity int) *SlubAllocator {
        return &SlubAllocator{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *SlubAllocator) Allocate() interface{ {
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
func (m *SlubAllocator) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
