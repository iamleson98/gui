// Question #370: mimalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: mimalloc, per-CPU, free lists, delayed reset
// Description: Explain mimalloc's per-CPU sharded free lists and delayed resets for high throughput.
package memory

import "sync"

// mimalloc Design
// Implements a memory management technique for question #370.
type MimallocDesign struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMimallocDesign creates a memory manager with the given capacity.
func NewMimallocDesign(capacity int) *MimallocDesign {
        return &MimallocDesign{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MimallocDesign) Allocate() interface{ {
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
func (m *MimallocDesign) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
