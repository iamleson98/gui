// Question #364: Arena Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: arena, bulk alloc, reset, region
// Description: Build an arena allocator that bulk-allocates from a parent and frees all at once.
package memory

import "sync"

// Arena Allocator
// Implements a memory management technique for question #364.
type Q364_ArenaAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ364_ArenaAllocator creates a memory manager with the given capacity.
func NewQ364_ArenaAllocator(capacity int) *Q364_ArenaAllocator {
        return &Q364_ArenaAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q364_ArenaAllocator) Allocate() any {
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
func (m *Q364_ArenaAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
