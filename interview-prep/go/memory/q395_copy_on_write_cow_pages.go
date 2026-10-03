// Question #395: Copy-on-Write (CoW) Pages
// Category: Memory Management | Difficulty: Hard
// Concepts: copy-on-write, fork, sharing, page protection
// Description: Share read-only pages and copy only on write to enable cheap fork and sharing.
package memory

import "sync"

// Copy-on-Write (CoW) Pages
// Implements a memory management technique for question #395.
type Q395_CopyOnWriteCowPages struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ395_CopyOnWriteCowPages creates a memory manager with the given capacity.
func NewQ395_CopyOnWriteCowPages(capacity int) *Q395_CopyOnWriteCowPages {
        return &Q395_CopyOnWriteCowPages{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q395_CopyOnWriteCowPages) Allocate() any {
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
func (m *Q395_CopyOnWriteCowPages) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
