// Question #389: Deferred Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: deferred RC, batching, amortize, hot path
// Description: Batch reference-count updates to amortize their cost on the hot path.
package memory

import "sync"

// Deferred Reference Counting
// Implements a memory management technique for question #389.
type DeferredReferenceCounting struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewDeferredReferenceCounting creates a memory manager with the given capacity.
func NewDeferredReferenceCounting(capacity int) *DeferredReferenceCounting {
        return &DeferredReferenceCounting{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *DeferredReferenceCounting) Allocate() interface{ {
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
func (m *DeferredReferenceCounting) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
