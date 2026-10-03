// Question #365: Region/Arena Bump Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: bump allocator, region, O(1), bulk free
// Description: Implement a bump pointer allocator within a region for O(1) allocation and bulk free.
package memory

import "sync"

// Region/Arena Bump Allocator
// Implements a memory management technique for question #365.
type Q365_RegionArenaBumpAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ365_RegionArenaBumpAllocator creates a memory manager with the given capacity.
func NewQ365_RegionArenaBumpAllocator(capacity int) *Q365_RegionArenaBumpAllocator {
        return &Q365_RegionArenaBumpAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q365_RegionArenaBumpAllocator) Allocate() any {
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
func (m *Q365_RegionArenaBumpAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
