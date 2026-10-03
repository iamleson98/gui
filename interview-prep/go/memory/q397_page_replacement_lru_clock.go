// Question #397: Page Replacement (LRU/Clock)
// Category: Memory Management | Difficulty: Hard
// Concepts: page replacement, LRU, clock, eviction
// Description: Implement LRU and clock page replacement policies for finite physical memory.
package memory

import "sync"

// Page Replacement (LRU/Clock)
// Implements a memory management technique for question #397.
type Q397_PageReplacementLruClock struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ397_PageReplacementLruClock creates a memory manager with the given capacity.
func NewQ397_PageReplacementLruClock(capacity int) *Q397_PageReplacementLruClock {
        return &Q397_PageReplacementLruClock{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q397_PageReplacementLruClock) Allocate() any {
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
func (m *Q397_PageReplacementLruClock) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
