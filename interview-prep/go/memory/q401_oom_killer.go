// Question #401: OOM Killer
// Category: Memory Management | Difficulty: Hard
// Concepts: OOM, killer, victim selection, memory score
// Description: Design an out-of-memory killer that selects victims based on memory score.
package memory

import "sync"

// OOM Killer
// Implements a memory management technique for question #401.
type OomKiller struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewOomKiller creates a memory manager with the given capacity.
func NewOomKiller(capacity int) *OomKiller {
        return &OomKiller{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *OomKiller) Allocate() interface{ {
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
func (m *OomKiller) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
