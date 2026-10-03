// Question #404: Compaction
// Category: Memory Management | Difficulty: Hard
// Concepts: compaction, forwarding, fragmentation, heap
// Description: Compact the heap to reduce external fragmentation via forwarding addresses.
package memory

import "sync"

// Compaction
// Implements a memory management technique for question #404.
type Compaction struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewCompaction creates a memory manager with the given capacity.
func NewCompaction(capacity int) *Compaction {
        return &Compaction{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Compaction) Allocate() interface{ {
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
func (m *Compaction) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
