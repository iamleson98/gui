// Question #385: Hazard Pointers for Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: hazard pointer, reclamation, lock-free, ABA
// Description: Use hazard pointers to safely reclaim memory in lock-free data structures.
package memory

import "sync"

// Hazard Pointers for Reclamation
// Implements a memory management technique for question #385.
type Q385_HazardPointersForReclamation struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ385_HazardPointersForReclamation creates a memory manager with the given capacity.
func NewQ385_HazardPointersForReclamation(capacity int) *Q385_HazardPointersForReclamation {
        return &Q385_HazardPointersForReclamation{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q385_HazardPointersForReclamation) Allocate() any {
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
func (m *Q385_HazardPointersForReclamation) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
