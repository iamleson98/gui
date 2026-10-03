// Question #395: Copy-on-Write (CoW) Pages
// Category: Memory Management | Difficulty: Hard
// Concepts: copy-on-write, fork, sharing, page protection
// Description: Share read-only pages and copy only on write to enable cheap fork and sharing.
package memory

import "sync"

// Copy-on-Write (CoW) Pages
// Implements a memory management technique for question #395.
type CopyOnWriteCowPages struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewCopyOnWriteCowPages creates a memory manager with the given capacity.
func NewCopyOnWriteCowPages(capacity int) *CopyOnWriteCowPages {
        return &CopyOnWriteCowPages{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *CopyOnWriteCowPages) Allocate() interface{ {
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
func (m *CopyOnWriteCowPages) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
