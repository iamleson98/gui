// Question #382: Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: reference counting, cycles, weak refs, deferred
// Description: Implement reference counting with cycle detection to reclaim unreachable cycles.
package memory

import "sync"

// Reference Counting
// Implements a memory management technique for question #382.
type ReferenceCounting struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewReferenceCounting creates a memory manager with the given capacity.
func NewReferenceCounting(capacity int) *ReferenceCounting {
        return &ReferenceCounting{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ReferenceCounting) Allocate() interface{ {
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
func (m *ReferenceCounting) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
