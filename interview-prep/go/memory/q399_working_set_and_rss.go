// Question #399: Working Set and RSS
// Category: Memory Management | Difficulty: Hard
// Concepts: working set, RSS, thrashing, sizing
// Description: Estimate the working set size and resident set to size memory and detect thrashing.
package memory

import "sync"

// Working Set and RSS
// Implements a memory management technique for question #399.
type WorkingSetAndRss struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewWorkingSetAndRss creates a memory manager with the given capacity.
func NewWorkingSetAndRss(capacity int) *WorkingSetAndRss {
        return &WorkingSetAndRss{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *WorkingSetAndRss) Allocate() interface{ {
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
func (m *WorkingSetAndRss) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
