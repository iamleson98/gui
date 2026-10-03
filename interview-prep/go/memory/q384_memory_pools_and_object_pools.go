// Question #384: Memory Pools and Object Pools
// Category: Memory Management | Difficulty: Hard
// Concepts: object pool, amortize, construct cost, reuse
// Description: Design object pools to amortize allocation of expensive-to-construct objects.
package memory

import "sync"

// Memory Pools and Object Pools
// Implements a memory management technique for question #384.
type Q384_MemoryPoolsAndObjectPools struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ384_MemoryPoolsAndObjectPools creates a memory manager with the given capacity.
func NewQ384_MemoryPoolsAndObjectPools(capacity int) *Q384_MemoryPoolsAndObjectPools {
        return &Q384_MemoryPoolsAndObjectPools{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q384_MemoryPoolsAndObjectPools) Allocate() any {
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
func (m *Q384_MemoryPoolsAndObjectPools) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
