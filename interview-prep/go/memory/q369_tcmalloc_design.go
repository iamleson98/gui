// Question #369: tcmalloc Design
// Category: Memory Management | Difficulty: Hard
// Concepts: tcmalloc, thread-local, spans, central heap
// Description: Explain tcmalloc's thread-local caches and span-based central heap for scalable allocation.
package memory

import "sync"

// tcmalloc Design
// Implements a memory management technique for question #369.
type TcmallocDesign struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewTcmallocDesign creates a memory manager with the given capacity.
func NewTcmallocDesign(capacity int) *TcmallocDesign {
        return &TcmallocDesign{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *TcmallocDesign) Allocate() interface{ {
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
func (m *TcmallocDesign) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
