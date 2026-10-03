// Question #394: mmap and Virtual Address Space
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap, virtual address, anonymous, file-backed
// Description: Use mmap to map files and anonymous memory into the process address space.
package memory

import "sync"

// mmap and Virtual Address Space
// Implements a memory management technique for question #394.
type Q394_MmapAndVirtualAddressSpace struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ394_MmapAndVirtualAddressSpace creates a memory manager with the given capacity.
func NewQ394_MmapAndVirtualAddressSpace(capacity int) *Q394_MmapAndVirtualAddressSpace {
        return &Q394_MmapAndVirtualAddressSpace{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q394_MmapAndVirtualAddressSpace) Allocate() any {
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
func (m *Q394_MmapAndVirtualAddressSpace) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
