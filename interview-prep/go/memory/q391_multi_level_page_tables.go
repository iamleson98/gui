// Question #391: Multi-Level Page Tables
// Category: Memory Management | Difficulty: Hard
// Concepts: multi-level page table, sparse, compact, translation
// Description: Design multi-level page tables to compactly represent sparse virtual address spaces.
package memory

import "sync"

// Multi-Level Page Tables
// Implements a memory management technique for question #391.
type MultiLevelPageTables struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMultiLevelPageTables creates a memory manager with the given capacity.
func NewMultiLevelPageTables(capacity int) *MultiLevelPageTables {
        return &MultiLevelPageTables{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MultiLevelPageTables) Allocate() interface{ {
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
func (m *MultiLevelPageTables) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
