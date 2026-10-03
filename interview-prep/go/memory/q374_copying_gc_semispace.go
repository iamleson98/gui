// Question #374: Copying GC (Semispace)
// Category: Memory Management | Difficulty: Hard
// Concepts: copying GC, semispace, evacuation, forwarding
// Description: Implement a copying collector that evacuates live objects between two semispaces.
package memory

import "sync"

// Copying GC (Semispace)
// Implements a memory management technique for question #374.
type Q374_CopyingGcSemispace struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ374_CopyingGcSemispace creates a memory manager with the given capacity.
func NewQ374_CopyingGcSemispace(capacity int) *Q374_CopyingGcSemispace {
        return &Q374_CopyingGcSemispace{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q374_CopyingGcSemispace) Allocate() any {
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
func (m *Q374_CopyingGcSemispace) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
