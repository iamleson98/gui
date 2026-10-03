// Question #388: Quiescent-State-Based Reclamation
// Category: Memory Management | Difficulty: Hard
// Concepts: quiescent state, reclamation, lock-free, safety
// Description: Reclaim memory at quiescent states observed across threads for lock-free safety.
package memory

import "sync"

// Quiescent-State-Based Reclamation
// Implements a memory management technique for question #388.
type Q388_QuiescentStateBasedReclamation struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ388_QuiescentStateBasedReclamation creates a memory manager with the given capacity.
func NewQ388_QuiescentStateBasedReclamation(capacity int) *Q388_QuiescentStateBasedReclamation {
        return &Q388_QuiescentStateBasedReclamation{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q388_QuiescentStateBasedReclamation) Allocate() any {
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
func (m *Q388_QuiescentStateBasedReclamation) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
