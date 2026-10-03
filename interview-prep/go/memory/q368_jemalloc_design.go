// Question #368: jemalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: jemalloc, size class, arena, thread cache
// Description: Explain jemalloc's size-class bins, arenas, and thread caches for low fragmentation.
package memory

import "sync"

// jemalloc Design
// Implements a memory management technique for question #368.
type JemallocDesign struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewJemallocDesign creates a memory manager with the given capacity.
func NewJemallocDesign(capacity int) *JemallocDesign {
        return &JemallocDesign{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *JemallocDesign) Allocate() interface{ {
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
func (m *JemallocDesign) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
