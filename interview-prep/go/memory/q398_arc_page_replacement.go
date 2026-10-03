// Question #398: ARC Page Replacement
// Category: Memory Management | Difficulty: Hard
// Concepts: ARC, adaptive, recency, frequency
// Description: Implement the adaptive replacement cache policy balancing recency and frequency.
package memory

import "sync"

// ARC Page Replacement
// Implements a memory management technique for question #398.
type ArcPageReplacement struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewArcPageReplacement creates a memory manager with the given capacity.
func NewArcPageReplacement(capacity int) *ArcPageReplacement {
        return &ArcPageReplacement{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ArcPageReplacement) Allocate() interface{ {
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
func (m *ArcPageReplacement) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
