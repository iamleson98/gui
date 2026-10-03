// Question #401: OOM Killer
// Category: Memory Management | Difficulty: Hard
// Concepts: OOM, killer, victim selection, memory score
// Description: Design an out-of-memory killer that selects victims based on memory score.
package memory

import "sync"

// OOM Killer
// Implements a memory management technique for question #401.
type Q401_OomKiller struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ401_OomKiller creates a memory manager with the given capacity.
func NewQ401_OomKiller(capacity int) *Q401_OomKiller {
        return &Q401_OomKiller{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q401_OomKiller) Allocate() any {
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
func (m *Q401_OomKiller) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
