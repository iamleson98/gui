// Question #403: Memory Fragmentation (External/Internal)
// Category: Memory Management | Difficulty: Hard
// Concepts: fragmentation, external, internal, coalescing
// Description: Distinguish external and internal fragmentation and mitigate each.
package memory

import "sync"

// Memory Fragmentation (External/Internal)
// Implements a memory management technique for question #403.
type MemoryFragmentationExternalInternal struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMemoryFragmentationExternalInternal creates a memory manager with the given capacity.
func NewMemoryFragmentationExternalInternal(capacity int) *MemoryFragmentationExternalInternal {
        return &MemoryFragmentationExternalInternal{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MemoryFragmentationExternalInternal) Allocate() interface{ {
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
func (m *MemoryFragmentationExternalInternal) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
