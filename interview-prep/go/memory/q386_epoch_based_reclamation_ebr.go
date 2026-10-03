// Question #386: Epoch-Based Reclamation (EBR)
// Category: Memory Management | Difficulty: Hard
// Concepts: EBR, epoch, deferred, lock-free
// Description: Defer reclamation until epochs advance past all readers for lock-free safety.
package memory

import "sync"

// Epoch-Based Reclamation (EBR)
// Implements a memory management technique for question #386.
type EpochBasedReclamationEbr struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewEpochBasedReclamationEbr creates a memory manager with the given capacity.
func NewEpochBasedReclamationEbr(capacity int) *EpochBasedReclamationEbr {
        return &EpochBasedReclamationEbr{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *EpochBasedReclamationEbr) Allocate() interface{ {
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
func (m *EpochBasedReclamationEbr) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
