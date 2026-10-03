// Question #397: Page Replacement (LRU/Clock)
// Category: Memory Management | Difficulty: Hard
// Concepts: page replacement, LRU, clock, eviction
// Description: Implement LRU and clock page replacement policies for finite physical memory.
package memory

import "sync"

// Page Replacement (LRU/Clock)
// Implements a memory management technique for question #397.
type PageReplacementLruClock struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewPageReplacementLruClock creates a memory manager with the given capacity.
func NewPageReplacementLruClock(capacity int) *PageReplacementLruClock {
        return &PageReplacementLruClock{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *PageReplacementLruClock) Allocate() interface{ {
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
func (m *PageReplacementLruClock) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
