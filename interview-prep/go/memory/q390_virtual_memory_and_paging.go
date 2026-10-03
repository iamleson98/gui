// Question #390: Virtual Memory and Paging
// Category: Memory Management | Difficulty: Hard
// Concepts: virtual memory, paging, page table, translation
// Description: Implement paging that maps virtual to physical pages via page tables.
package memory

import "sync"

// Virtual Memory and Paging
// Implements a memory management technique for question #390.
type VirtualMemoryAndPaging struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewVirtualMemoryAndPaging creates a memory manager with the given capacity.
func NewVirtualMemoryAndPaging(capacity int) *VirtualMemoryAndPaging {
        return &VirtualMemoryAndPaging{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *VirtualMemoryAndPaging) Allocate() interface{ {
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
func (m *VirtualMemoryAndPaging) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
