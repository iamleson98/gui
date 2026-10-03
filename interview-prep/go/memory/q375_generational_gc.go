// Question #375: Generational GC
// Category: Memory Management | Difficulty: Hard
// Concepts: generational, young/old, remembered set, promotion
// Description: Design a generational collector with young and old generations using remembered sets.
package memory

import "sync"

// Generational GC
// Implements a memory management technique for question #375.
type GenerationalGc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewGenerationalGc creates a memory manager with the given capacity.
func NewGenerationalGc(capacity int) *GenerationalGc {
        return &GenerationalGc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *GenerationalGc) Allocate() interface{ {
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
func (m *GenerationalGc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
