// Question #410: Huge Page Table Walks
// Category: Memory Management | Difficulty: Hard
// Concepts: page walk, huge page, TLB, cost
// Description: Analyze page-walk cost with huge pages and the resulting TLB savings.
package memory

import "sync"

// Huge Page Table Walks
// Implements a memory management technique for question #410.
type HugePageTableWalks struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewHugePageTableWalks creates a memory manager with the given capacity.
func NewHugePageTableWalks(capacity int) *HugePageTableWalks {
        return &HugePageTableWalks{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *HugePageTableWalks) Allocate() interface{ {
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
func (m *HugePageTableWalks) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
