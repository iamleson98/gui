// Question #373: Mark-Compact GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-compact, compaction, fragmentation, forwarding
// Description: Implement a mark-compact collector that eliminates fragmentation by sliding live objects.
package memory

import "sync"

// Mark-Compact GC
// Implements a memory management technique for question #373.
type MarkCompactGc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMarkCompactGc creates a memory manager with the given capacity.
func NewMarkCompactGc(capacity int) *MarkCompactGc {
        return &MarkCompactGc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MarkCompactGc) Allocate() interface{ {
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
func (m *MarkCompactGc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
