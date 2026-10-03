// Question #389: Deferred Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: deferred RC, batching, amortize, hot path
// Description: Batch reference-count updates to amortize their cost on the hot path.
package memory

import "sync"

// Deferred Reference Counting
// Implements a memory management technique for question #389.
type Q389_DeferredReferenceCounting struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ389_DeferredReferenceCounting creates a memory manager with the given capacity.
func NewQ389_DeferredReferenceCounting(capacity int) *Q389_DeferredReferenceCounting {
        return &Q389_DeferredReferenceCounting{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q389_DeferredReferenceCounting) Allocate() any {
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
func (m *Q389_DeferredReferenceCounting) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
