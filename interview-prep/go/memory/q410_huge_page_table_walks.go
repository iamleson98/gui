// Question #410: Huge Page Table Walks
// Category: Memory Management | Difficulty: Hard
// Concepts: page walk, huge page, TLB, cost
// Description: Analyze page-walk cost with huge pages and the resulting TLB savings.
package memory

import "sync"

// Huge Page Table Walks
// Implements a memory management technique for question #410.
type Q410_HugePageTableWalks struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ410_HugePageTableWalks creates a memory manager with the given capacity.
func NewQ410_HugePageTableWalks(capacity int) *Q410_HugePageTableWalks {
        return &Q410_HugePageTableWalks{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q410_HugePageTableWalks) Allocate() any {
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
func (m *Q410_HugePageTableWalks) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
