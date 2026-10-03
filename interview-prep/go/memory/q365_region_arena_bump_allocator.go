// Question #365: Region/Arena Bump Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: bump allocator, region, O(1), bulk free
// Description: Implement a bump pointer allocator within a region for O(1) allocation and bulk free.
package memory

import "sync"

// Region/Arena Bump Allocator
// Implements a memory management technique for question #365.
type RegionArenaBumpAllocator struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewRegionArenaBumpAllocator creates a memory manager with the given capacity.
func NewRegionArenaBumpAllocator(capacity int) *RegionArenaBumpAllocator {
        return &RegionArenaBumpAllocator{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *RegionArenaBumpAllocator) Allocate() interface{ {
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
func (m *RegionArenaBumpAllocator) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
