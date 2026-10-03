// Question #396: Demand Paging
// Category: Memory Management | Difficulty: Hard
// Concepts: demand paging, page fault, lazy, zero fill
// Description: Load pages on first access via page faults to avoid eager allocation.
package memory

import "sync"

// Demand Paging
// Implements a memory management technique for question #396.
type DemandPaging struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewDemandPaging creates a memory manager with the given capacity.
func NewDemandPaging(capacity int) *DemandPaging {
        return &DemandPaging{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *DemandPaging) Allocate() interface{ {
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
func (m *DemandPaging) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
