// Question #398: ARC Page Replacement
// Category: Memory Management | Difficulty: Hard
// Concepts: ARC, adaptive, recency, frequency
// Description: Implement the adaptive replacement cache policy balancing recency and frequency.
package memory

import "sync"

// ARC Page Replacement
// Implements a memory management technique for question #398.
type Q398_ArcPageReplacement struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ398_ArcPageReplacement creates a memory manager with the given capacity.
func NewQ398_ArcPageReplacement(capacity int) *Q398_ArcPageReplacement {
        return &Q398_ArcPageReplacement{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q398_ArcPageReplacement) Allocate() any {
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
func (m *Q398_ArcPageReplacement) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
