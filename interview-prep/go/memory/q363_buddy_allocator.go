// Question #363: Buddy Allocator
// Category: Memory Management | Difficulty: Hard
// Concepts: buddy, coalescing, power-of-two, split
// Description: Implement a binary buddy allocator splitting and coalescing power-of-two blocks.
package memory

import "sync"

// Buddy Allocator
// Implements a memory management technique for question #363.
type BuddyAllocator struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewBuddyAllocator creates a memory manager with the given capacity.
func NewBuddyAllocator(capacity int) *BuddyAllocator {
        return &BuddyAllocator{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *BuddyAllocator) Allocate() interface{ {
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
func (m *BuddyAllocator) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
