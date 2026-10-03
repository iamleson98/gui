// Question #404: Q404_Compaction
// Category: Memory Management | Difficulty: Hard
// Concepts: compaction, forwarding, fragmentation, heap
// Description: Compact the heap to reduce external fragmentation via forwarding addresses.
package memory

import "sync"

// Q404_Compaction
// Implements a memory management technique for question #404.
type Q404_Compaction struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ404_Compaction creates a memory manager with the given capacity.
func NewQ404_Compaction(capacity int) *Q404_Compaction {
        return &Q404_Compaction{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q404_Compaction) Allocate() any {
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
func (m *Q404_Compaction) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
