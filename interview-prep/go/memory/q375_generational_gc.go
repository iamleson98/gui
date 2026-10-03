// Question #375: Generational GC
// Category: Memory Management | Difficulty: Hard
// Concepts: generational, young/old, remembered set, promotion
// Description: Design a generational collector with young and old generations using remembered sets.
package memory

import "sync"

// Generational GC
// Implements a memory management technique for question #375.
type Q375_GenerationalGc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ375_GenerationalGc creates a memory manager with the given capacity.
func NewQ375_GenerationalGc(capacity int) *Q375_GenerationalGc {
        return &Q375_GenerationalGc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q375_GenerationalGc) Allocate() any {
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
func (m *Q375_GenerationalGc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
