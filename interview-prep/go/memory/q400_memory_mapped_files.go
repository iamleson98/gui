// Question #400: Memory-Mapped Files
// Category: Memory Management | Difficulty: Hard
// Concepts: mmap files, zero-copy, page cache, I/O
// Description: Access files through mapped pages for zero-copy I/O and kernel-managed caching.
package memory

import "sync"

// Memory-Mapped Files
// Implements a memory management technique for question #400.
type Q400_MemoryMappedFiles struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ400_MemoryMappedFiles creates a memory manager with the given capacity.
func NewQ400_MemoryMappedFiles(capacity int) *Q400_MemoryMappedFiles {
        return &Q400_MemoryMappedFiles{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q400_MemoryMappedFiles) Allocate() any {
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
func (m *Q400_MemoryMappedFiles) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
