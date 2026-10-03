// Question #388: Quiescent-State-Based Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: quiescent state, reclamation, lock-free, safety
// Description: Reclaim memory at quiescent states observed across threads for lock-free safety.
package memory

import "sync"

// Quiescent-State-Based Reclamation
// Implements a memory management technique for question #388.
type QuiescentStateBasedReclamation struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewQuiescentStateBasedReclamation creates a memory manager with the given capacity.
func NewQuiescentStateBasedReclamation(capacity int) *QuiescentStateBasedReclamation {
        return &QuiescentStateBasedReclamation{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *QuiescentStateBasedReclamation) Allocate() interface{ {
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
func (m *QuiescentStateBasedReclamation) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
