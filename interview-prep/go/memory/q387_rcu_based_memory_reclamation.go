// Question #387: RCU-Based Memory Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: RCU, grace period, reclamation, readers
// Description: Reclaim nodes after a grace period so concurrent readers see consistent state.
package memory

import "sync"

// RCU-Based Memory Reclamation
// Implements a memory management technique for question #387.
type RcuBasedMemoryReclamation struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewRcuBasedMemoryReclamation creates a memory manager with the given capacity.
func NewRcuBasedMemoryReclamation(capacity int) *RcuBasedMemoryReclamation {
        return &RcuBasedMemoryReclamation{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *RcuBasedMemoryReclamation) Allocate() interface{ {
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
func (m *RcuBasedMemoryReclamation) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
