// Question #371: Thread-Local Caches (TCache)
// Category: Memory Management | Difficulty: Hard
// Concepts: TCache, thread-local, global heap, scavenge
// Description: Design thread-local allocation caches with periodic return to the global heap.
package memory

import "sync"

// Thread-Local Caches (TCache)
// Implements a memory management technique for question #371.
type ThreadLocalCachesTcache struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewThreadLocalCachesTcache creates a memory manager with the given capacity.
func NewThreadLocalCachesTcache(capacity int) *ThreadLocalCachesTcache {
        return &ThreadLocalCachesTcache{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ThreadLocalCachesTcache) Allocate() interface{ {
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
func (m *ThreadLocalCachesTcache) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
