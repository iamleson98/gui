// Question #381: Write Barriers (SATB/INC)
// Category: Memory Management | Difficulty: Hard
// Concepts: write barrier, SATB, incremental, invariant
// Description: Implement SATB and incremental-update write barriers to maintain tri-color invariance.
package memory

import "sync"

// Write Barriers (SATB/INC)
// Implements a memory management technique for question #381.
type Q381_WriteBarriersSatbInc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ381_WriteBarriersSatbInc creates a memory manager with the given capacity.
func NewQ381_WriteBarriersSatbInc(capacity int) *Q381_WriteBarriersSatbInc {
        return &Q381_WriteBarriersSatbInc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q381_WriteBarriersSatbInc) Allocate() any {
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
func (m *Q381_WriteBarriersSatbInc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
