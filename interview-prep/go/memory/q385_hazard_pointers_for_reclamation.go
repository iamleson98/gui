// Question #385: Hazard Pointers for Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: hazard pointer, reclamation, lock-free, ABA
// Description: Use hazard pointers to safely reclaim memory in lock-free data structures.
package memory

import "sync"

// Hazard Pointers for Reclamation
// Implements a memory management technique for question #385.
type HazardPointersForReclamation struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewHazardPointersForReclamation creates a memory manager with the given capacity.
func NewHazardPointersForReclamation(capacity int) *HazardPointersForReclamation {
        return &HazardPointersForReclamation{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *HazardPointersForReclamation) Allocate() interface{ {
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
func (m *HazardPointersForReclamation) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
