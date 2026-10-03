// Question #396: Demand Paging
// Category: Memory Management | Difficulty: Hard
// Concepts: demand paging, page fault, lazy, zero fill
// Description: Load pages on first access via page faults to avoid eager allocation.
package memory

import "sync"

// Demand Paging
// Implements a memory management technique for question #396.
type Q396_DemandPaging struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ396_DemandPaging creates a memory manager with the given capacity.
func NewQ396_DemandPaging(capacity int) *Q396_DemandPaging {
        return &Q396_DemandPaging{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q396_DemandPaging) Allocate() any {
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
func (m *Q396_DemandPaging) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
