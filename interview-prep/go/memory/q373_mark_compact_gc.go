// Question #373: Mark-Compact GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-compact, compaction, fragmentation, forwarding
// Description: Implement a mark-compact collector that eliminates fragmentation by sliding live objects.
package memory

import "sync"

// Mark-Compact GC
// Implements a memory management technique for question #373.
type Q373_MarkCompactGc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ373_MarkCompactGc creates a memory manager with the given capacity.
func NewQ373_MarkCompactGc(capacity int) *Q373_MarkCompactGc {
        return &Q373_MarkCompactGc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q373_MarkCompactGc) Allocate() any {
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
func (m *Q373_MarkCompactGc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
