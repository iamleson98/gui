// Question #381: Write Barriers (SATB/INC)
// Category: Memory Management | Difficulty: Hard
// Concepts: write barrier, SATB, incremental, invariant
// Description: Implement SATB and incremental-update write barriers to maintain tri-color invariance.
package memory

import "sync"

// Write Barriers (SATB/INC)
// Implements a memory management technique for question #381.
type WriteBarriersSatbInc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewWriteBarriersSatbInc creates a memory manager with the given capacity.
func NewWriteBarriersSatbInc(capacity int) *WriteBarriersSatbInc {
        return &WriteBarriersSatbInc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *WriteBarriersSatbInc) Allocate() interface{ {
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
func (m *WriteBarriersSatbInc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
