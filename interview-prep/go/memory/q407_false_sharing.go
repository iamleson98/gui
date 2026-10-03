// Question #407: False Sharing
// Category: Memory Management | Difficulty: Hard
// Concepts: false sharing, cache line, padding, contention
// Description: Diagnose false sharing on shared mutable fields in the same cache line and pad them apart.
package memory

import "sync"

// False Sharing
// Implements a memory management technique for question #407.
type FalseSharing struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewFalseSharing creates a memory manager with the given capacity.
func NewFalseSharing(capacity int) *FalseSharing {
        return &FalseSharing{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *FalseSharing) Allocate() interface{ {
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
func (m *FalseSharing) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
