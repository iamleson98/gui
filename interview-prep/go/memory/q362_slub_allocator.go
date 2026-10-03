// Question #362: Slub Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: SLUB, per-CPU, freelist, kernel
// Description: Explain the SLUB allocator's simpler, per-CPU design replacing the classic slab allocator.
package memory

import "sync"

// Slub Allocator
// Implements a memory management technique for question #362.
type Q362_SlubAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ362_SlubAllocator creates a memory manager with the given capacity.
func NewQ362_SlubAllocator(capacity int) *Q362_SlubAllocator {
        return &Q362_SlubAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q362_SlubAllocator) Allocate() any {
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
func (m *Q362_SlubAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
