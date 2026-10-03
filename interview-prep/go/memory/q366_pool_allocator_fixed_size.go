// Question #366: Pool Allocator (Fixed-Size)
// Category: Memory Management | Difficulty: Hard
// Concepts: pool, fixed-size, free list, O(1)
// Description: Implement a fixed-size pool allocator using a free list for constant-time alloc/free.
package memory

import "sync"

// Pool Allocator (Fixed-Size)
// Implements a memory management technique for question #366.
type Q366_PoolAllocatorFixedSize struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ366_PoolAllocatorFixedSize creates a memory manager with the given capacity.
func NewQ366_PoolAllocatorFixedSize(capacity int) *Q366_PoolAllocatorFixedSize {
        return &Q366_PoolAllocatorFixedSize{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q366_PoolAllocatorFixedSize) Allocate() any {
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
func (m *Q366_PoolAllocatorFixedSize) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
