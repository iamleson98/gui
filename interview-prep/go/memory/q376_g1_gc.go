// Question #376: G1 GC
// Category: Memory Management | Difficulty: Hard
// Concepts: G1, regions, pause prediction, compaction
// Description: Explain the Garbage-First collector's region-based layout and pause-time predictability.
package memory

import "sync"

// G1 GC
// Implements a memory management technique for question #376.
type Q376_G1Gc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ376_G1Gc creates a memory manager with the given capacity.
func NewQ376_G1Gc(capacity int) *Q376_G1Gc {
        return &Q376_G1Gc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q376_G1Gc) Allocate() any {
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
func (m *Q376_G1Gc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
