// Question #402: Overcommit and OOM
// Category: Memory Management | Difficulty: Hard
// Concepts: overcommit, commit limit, OOM, accounting
// Description: Reason about memory overcommit, commit limits, and the consequences for OOM.
package memory

import "sync"

// Overcommit and OOM
// Implements a memory management technique for question #402.
type OvercommitAndOom struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewOvercommitAndOom creates a memory manager with the given capacity.
func NewOvercommitAndOom(capacity int) *OvercommitAndOom {
        return &OvercommitAndOom{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *OvercommitAndOom) Allocate() interface{ {
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
func (m *OvercommitAndOom) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
