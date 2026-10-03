// Question #376: G1 GC
// Category: Memory Management | Difficulty: Hard
// Concepts: G1, regions, pause prediction, compaction
// Description: Explain the Garbage-First collector's region-based layout and pause-time predictability.
package memory

import "sync"

// G1 GC
// Implements a memory management technique for question #376.
type G1Gc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewG1Gc creates a memory manager with the given capacity.
func NewG1Gc(capacity int) *G1Gc {
        return &G1Gc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *G1Gc) Allocate() interface{ {
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
func (m *G1Gc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
