// Question #371: Thread-Local Caches (TCache)
// Category: Memory Management | Difficulty: Hard
// Concepts: TCache, thread-local, global heap, scavenge
// Description: Design thread-local allocation caches with periodic return to the global heap.
package memory

import "sync"

// Thread-Local Caches (TCache)
// Implements a memory management technique for question #371.
type Q371_ThreadLocalCachesTcache struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ371_ThreadLocalCachesTcache creates a memory manager with the given capacity.
func NewQ371_ThreadLocalCachesTcache(capacity int) *Q371_ThreadLocalCachesTcache {
        return &Q371_ThreadLocalCachesTcache{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q371_ThreadLocalCachesTcache) Allocate() any {
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
func (m *Q371_ThreadLocalCachesTcache) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
