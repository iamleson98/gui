// Question #394: mmap and Virtual Address Space
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap, virtual address, anonymous, file-backed
// Description: Use mmap to map files and anonymous memory into the process address space.
package memory

import "sync"

// mmap and Virtual Address Space
// Implements a memory management technique for question #394.
type MmapAndVirtualAddressSpace struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMmapAndVirtualAddressSpace creates a memory manager with the given capacity.
func NewMmapAndVirtualAddressSpace(capacity int) *MmapAndVirtualAddressSpace {
        return &MmapAndVirtualAddressSpace{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MmapAndVirtualAddressSpace) Allocate() interface{ {
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
func (m *MmapAndVirtualAddressSpace) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
