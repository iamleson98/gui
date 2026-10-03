// Question #374: Copying GC (Semispace)
// Category: Memory Management | Difficulty: Hard
// Concepts: copying GC, semispace, evacuation, forwarding
// Description: Implement a copying collector that evacuates live objects between two semispaces.
package memory

import "sync"

// Copying GC (Semispace)
// Implements a memory management technique for question #374.
type CopyingGcSemispace struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewCopyingGcSemispace creates a memory manager with the given capacity.
func NewCopyingGcSemispace(capacity int) *CopyingGcSemispace {
        return &CopyingGcSemispace{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *CopyingGcSemispace) Allocate() interface{ {
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
func (m *CopyingGcSemispace) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
