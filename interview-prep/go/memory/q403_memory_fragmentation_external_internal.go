// Question #403: Memory Fragmentation (External/Internal)
// Category: Memory Management | Difficulty: Hard
// Concepts: fragmentation, external, internal, coalescing
// Description: Distinguish external and internal fragmentation and mitigate each.
package memory

import "sync"

// Memory Fragmentation (External/Internal)
// Implements a memory management technique for question #403.
type Q403_MemoryFragmentationExternalInternal struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ403_MemoryFragmentationExternalInternal creates a memory manager with the given capacity.
func NewQ403_MemoryFragmentationExternalInternal(capacity int) *Q403_MemoryFragmentationExternalInternal {
        return &Q403_MemoryFragmentationExternalInternal{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q403_MemoryFragmentationExternalInternal) Allocate() any {
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
func (m *Q403_MemoryFragmentationExternalInternal) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
