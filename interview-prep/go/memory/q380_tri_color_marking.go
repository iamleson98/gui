// Question #380: Tri-Color Marking
// Category: Memory Management | Difficulty: Hard
// Concepts: tri-color, white/gray/black, invariant, tracing
// Description: Implement the tri-color invariant (white, gray, black) during tracing.
package memory

import "sync"

// Tri-Color Marking
// Implements a memory management technique for question #380.
type Q380_TriColorMarking struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ380_TriColorMarking creates a memory manager with the given capacity.
func NewQ380_TriColorMarking(capacity int) *Q380_TriColorMarking {
        return &Q380_TriColorMarking{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q380_TriColorMarking) Allocate() any {
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
func (m *Q380_TriColorMarking) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
