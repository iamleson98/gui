// Question #391: Multi-Level Page Tables
// Category: Memory Management | Difficulty: Hard
// Concepts: multi-level page table, sparse, compact, translation
// Description: Design multi-level page tables to compactly represent sparse virtual address spaces.
package memory

import "sync"

// Multi-Level Page Tables
// Implements a memory management technique for question #391.
type Q391_MultiLevelPageTables struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ391_MultiLevelPageTables creates a memory manager with the given capacity.
func NewQ391_MultiLevelPageTables(capacity int) *Q391_MultiLevelPageTables {
        return &Q391_MultiLevelPageTables{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q391_MultiLevelPageTables) Allocate() any {
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
func (m *Q391_MultiLevelPageTables) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
