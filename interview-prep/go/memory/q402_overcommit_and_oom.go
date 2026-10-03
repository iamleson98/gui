// Question #402: Overcommit and OOM
// Category: Memory Management | Difficulty: Hard
// Concepts: overcommit, commit limit, OOM, accounting
// Description: Reason about memory overcommit, commit limits, and the consequences for OOM.
package memory

import "sync"

// Overcommit and OOM
// Implements a memory management technique for question #402.
type Q402_OvercommitAndOom struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ402_OvercommitAndOom creates a memory manager with the given capacity.
func NewQ402_OvercommitAndOom(capacity int) *Q402_OvercommitAndOom {
        return &Q402_OvercommitAndOom{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q402_OvercommitAndOom) Allocate() any {
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
func (m *Q402_OvercommitAndOom) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
