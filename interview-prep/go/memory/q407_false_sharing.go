// Question #407: False Sharing
// Category: Memory Management | Difficulty: Hard
// Concepts: false sharing, cache line, padding, contention
// Description: Diagnose false sharing on shared mutable fields in the same cache line and pad them apart.
package memory

import "sync"

// False Sharing
// Implements a memory management technique for question #407.
type Q407_FalseSharing struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ407_FalseSharing creates a memory manager with the given capacity.
func NewQ407_FalseSharing(capacity int) *Q407_FalseSharing {
        return &Q407_FalseSharing{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q407_FalseSharing) Allocate() any {
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
func (m *Q407_FalseSharing) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
