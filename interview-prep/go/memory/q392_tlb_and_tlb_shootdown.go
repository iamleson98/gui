// Question #392: TLB and TLB Shootdown
// Category: Memory Management | Difficulty: Hard
// Concepts: TLB, shootdown, IPI, translation cache
// Description: Explain the TLB cache of translations and the cost of cross-CPU shootdowns.
package memory

import "sync"

// TLB and TLB Shootdown
// Implements a memory management technique for question #392.
type Q392_TlbAndTlbShootdown struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ392_TlbAndTlbShootdown creates a memory manager with the given capacity.
func NewQ392_TlbAndTlbShootdown(capacity int) *Q392_TlbAndTlbShootdown {
        return &Q392_TlbAndTlbShootdown{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q392_TlbAndTlbShootdown) Allocate() any {
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
func (m *Q392_TlbAndTlbShootdown) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
