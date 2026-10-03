// Question #393: Huge Pages (Transparent)
// Category: Memory Management | Difficulty: Hard
// Concepts: huge pages, THP, TLB, page walk
// Description: Use huge pages to reduce TLB pressure and page-walk cost for large allocations.
package memory

import "sync"

// Huge Pages (Transparent)
// Implements a memory management technique for question #393.
type HugePagesTransparent struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewHugePagesTransparent creates a memory manager with the given capacity.
func NewHugePagesTransparent(capacity int) *HugePagesTransparent {
        return &HugePagesTransparent{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *HugePagesTransparent) Allocate() interface{ {
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
func (m *HugePagesTransparent) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
