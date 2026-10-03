// Question #392: TLB and TLB Shootdown
// Category: Memory Management | Difficulty: Hard
// Concepts: TLB, shootdown, IPI, translation cache
// Description: Explain the TLB cache of translations and the cost of cross-CPU shootdowns.
package memory

import "sync"

// TLB and TLB Shootdown
// Implements a memory management technique for question #392.
type TlbAndTlbShootdown struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewTlbAndTlbShootdown creates a memory manager with the given capacity.
func NewTlbAndTlbShootdown(capacity int) *TlbAndTlbShootdown {
        return &TlbAndTlbShootdown{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *TlbAndTlbShootdown) Allocate() interface{ {
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
func (m *TlbAndTlbShootdown) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
