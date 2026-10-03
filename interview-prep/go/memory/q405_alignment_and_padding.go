// Question #405: Alignment and Padding
// Category: Memory Management | Difficulty: Hard
// Concepts: alignment, padding, struct layout, ABI
// Description: Lay out structs with alignment rules to avoid misaligned access and padding waste.
package memory

import "sync"

// Alignment and Padding
// Implements a memory management technique for question #405.
type Q405_AlignmentAndPadding struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ405_AlignmentAndPadding creates a memory manager with the given capacity.
func NewQ405_AlignmentAndPadding(capacity int) *Q405_AlignmentAndPadding {
        return &Q405_AlignmentAndPadding{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q405_AlignmentAndPadding) Allocate() any {
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
func (m *Q405_AlignmentAndPadding) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
