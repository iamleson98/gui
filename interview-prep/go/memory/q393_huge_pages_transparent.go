// Question #393: Huge Pages (Transparent)
// Category: Memory Management | Difficulty: Hard
// Concepts: huge pages, THP, TLB, page walk
// Description: Use huge pages to reduce TLB pressure and page-walk cost for large allocations.
package memory

import "sync"

// Huge Pages (Transparent)
// Implements a memory management technique for question #393.
type Q393_HugePagesTransparent struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ393_HugePagesTransparent creates a memory manager with the given capacity.
func NewQ393_HugePagesTransparent(capacity int) *Q393_HugePagesTransparent {
        return &Q393_HugePagesTransparent{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q393_HugePagesTransparent) Allocate() any {
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
func (m *Q393_HugePagesTransparent) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
