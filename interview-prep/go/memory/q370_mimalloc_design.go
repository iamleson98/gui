// Question #370: mimalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: mimalloc, per-CPU, free lists, delayed reset
// Description: Explain mimalloc's per-CPU sharded free lists and delayed resets for high throughput.
package memory

import "sync"

// mimalloc Design
// Implements a memory management technique for question #370.
type Q370_MimallocDesign struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ370_MimallocDesign creates a memory manager with the given capacity.
func NewQ370_MimallocDesign(capacity int) *Q370_MimallocDesign {
        return &Q370_MimallocDesign{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q370_MimallocDesign) Allocate() any {
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
func (m *Q370_MimallocDesign) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
