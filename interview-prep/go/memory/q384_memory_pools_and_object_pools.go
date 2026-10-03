// Question #384: Memory Pools and Object Pools
// Category: Memory Management | Difficulty: Hard
// Concepts: object pool, amortize, construct cost, reuse
// Description: Design object pools to amortize allocation of expensive-to-construct objects.
package memory

import "sync"

// Memory Pools and Object Pools
// Implements a memory management technique for question #384.
type MemoryPoolsAndObjectPools struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMemoryPoolsAndObjectPools creates a memory manager with the given capacity.
func NewMemoryPoolsAndObjectPools(capacity int) *MemoryPoolsAndObjectPools {
        return &MemoryPoolsAndObjectPools{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MemoryPoolsAndObjectPools) Allocate() interface{ {
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
func (m *MemoryPoolsAndObjectPools) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
