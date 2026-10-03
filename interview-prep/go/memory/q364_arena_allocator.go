// Question #364: Arena Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: arena, bulk alloc, reset, region
// Description: Build an arena allocator that bulk-allocates from a parent and frees all at once.
package memory

import "sync"

// Arena Allocator
// Implements a memory management technique for question #364.
type ArenaAllocator struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewArenaAllocator creates a memory manager with the given capacity.
func NewArenaAllocator(capacity int) *ArenaAllocator {
        return &ArenaAllocator{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ArenaAllocator) Allocate() interface{ {
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
func (m *ArenaAllocator) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
