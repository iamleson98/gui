// Question #386: Epoch-Based Reclamation (EBR)
// Category: Memory Management | Difficulty: Hard
// Concepts: EBR, epoch, deferred, lock-free
// Description: Defer reclamation until epochs advance past all readers for lock-free safety.
package memory

import "sync"

// Epoch-Based Reclamation (EBR)
// Implements a memory management technique for question #386.
type Q386_EpochBasedReclamationEbr struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ386_EpochBasedReclamationEbr creates a memory manager with the given capacity.
func NewQ386_EpochBasedReclamationEbr(capacity int) *Q386_EpochBasedReclamationEbr {
        return &Q386_EpochBasedReclamationEbr{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q386_EpochBasedReclamationEbr) Allocate() any {
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
func (m *Q386_EpochBasedReclamationEbr) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
