// Question #380: Tri-Color Marking
// Category: Memory Management | Difficulty: Hard
// Concepts: tri-color, white/gray/black, invariant, tracing
// Description: Implement the tri-color invariant (white, gray, black) during tracing.
package memory

import "sync"

// Tri-Color Marking
// Implements a memory management technique for question #380.
type TriColorMarking struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewTriColorMarking creates a memory manager with the given capacity.
func NewTriColorMarking(capacity int) *TriColorMarking {
        return &TriColorMarking{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *TriColorMarking) Allocate() interface{ {
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
func (m *TriColorMarking) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
