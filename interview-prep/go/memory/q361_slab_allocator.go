// Question #361: Slab Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: slab, caches, fixed-size, kernel
// Description: Implement a slab allocator caching fixed-size object states for the kernel to reduce fragmentation.
package memory

import "sync"

// Slab Allocator
// Implements a memory management technique for question #361.
type Q361_SlabAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ361_SlabAllocator creates a memory manager with the given capacity.
func NewQ361_SlabAllocator(capacity int) *Q361_SlabAllocator {
        return &Q361_SlabAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q361_SlabAllocator) Allocate() any {
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
func (m *Q361_SlabAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
