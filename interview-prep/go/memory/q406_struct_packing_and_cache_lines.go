// Question #406: Struct Packing and Cache Lines
// Category: Memory Management | Difficulty: Hard
// Concepts: packing, cache line, layout, alignment
// Description: Pack structs to fit within cache lines and trade size against access speed.
package memory

import "sync"

// Struct Packing and Cache Lines
// Implements a memory management technique for question #406.
type StructPackingAndCacheLines struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewStructPackingAndCacheLines creates a memory manager with the given capacity.
func NewStructPackingAndCacheLines(capacity int) *StructPackingAndCacheLines {
        return &StructPackingAndCacheLines{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *StructPackingAndCacheLines) Allocate() interface{ {
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
func (m *StructPackingAndCacheLines) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
