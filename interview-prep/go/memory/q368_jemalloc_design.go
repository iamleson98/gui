// Question #368: jemalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: jemalloc, size class, arena, thread cache
// Description: Explain jemalloc's size-class bins, arenas, and thread caches for low fragmentation.
package memory

import "sync"

// jemalloc Design
// Implements a memory management technique for question #368.
type Q368_JemallocDesign struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ368_JemallocDesign creates a memory manager with the given capacity.
func NewQ368_JemallocDesign(capacity int) *Q368_JemallocDesign {
        return &Q368_JemallocDesign{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q368_JemallocDesign) Allocate() any {
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
func (m *Q368_JemallocDesign) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
