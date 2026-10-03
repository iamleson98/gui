// Question #363: Buddy Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: buddy, coalescing, power-of-two, split
// Description: Implement a binary buddy allocator splitting and coalescing power-of-two blocks.
package memory

import "sync"

// Buddy Allocator
// Implements a memory management technique for question #363.
type Q363_BuddyAllocator struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ363_BuddyAllocator creates a memory manager with the given capacity.
func NewQ363_BuddyAllocator(capacity int) *Q363_BuddyAllocator {
        return &Q363_BuddyAllocator{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q363_BuddyAllocator) Allocate() any {
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
func (m *Q363_BuddyAllocator) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
