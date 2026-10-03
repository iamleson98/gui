// Question #408: Cache-Friendly Data Layout (SoA vs AoS)
// Category: Memory Management | Difficulty: Hard
// Concepts: SoA, AoS, SIMD, cache
// Description: Choose between array-of-structs and struct-of-arrays for SIMD and cache efficiency.
package memory

import "sync"

// Cache-Friendly Data Layout (SoA vs AoS)
// Implements a memory management technique for question #408.
type CacheFriendlyDataLayoutSoaVsAos struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewCacheFriendlyDataLayoutSoaVsAos creates a memory manager with the given capacity.
func NewCacheFriendlyDataLayoutSoaVsAos(capacity int) *CacheFriendlyDataLayoutSoaVsAos {
        return &CacheFriendlyDataLayoutSoaVsAos{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *CacheFriendlyDataLayoutSoaVsAos) Allocate() interface{ {
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
func (m *CacheFriendlyDataLayoutSoaVsAos) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
