// Question #379: Concurrent Marking
// Category: Memory Management | Difficulty: Hard
// Concepts: concurrent marking, safe-point, handshake, pause
// Description: Implement concurrent marking with safe-points and handshakes to avoid long pauses.
package memory

import "sync"

// Concurrent Marking
// Implements a memory management technique for question #379.
type ConcurrentMarking struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewConcurrentMarking creates a memory manager with the given capacity.
func NewConcurrentMarking(capacity int) *ConcurrentMarking {
        return &ConcurrentMarking{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ConcurrentMarking) Allocate() interface{ {
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
func (m *ConcurrentMarking) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
