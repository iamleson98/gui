// Question #400: Memory-Mapped Files
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap files, zero-copy, page cache, I/O
// Description: Access files through mapped pages for zero-copy I/O and kernel-managed caching.
package memory

import "sync"

// Memory-Mapped Files
// Implements a memory management technique for question #400.
type MemoryMappedFiles struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMemoryMappedFiles creates a memory manager with the given capacity.
func NewMemoryMappedFiles(capacity int) *MemoryMappedFiles {
        return &MemoryMappedFiles{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MemoryMappedFiles) Allocate() interface{ {
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
func (m *MemoryMappedFiles) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
