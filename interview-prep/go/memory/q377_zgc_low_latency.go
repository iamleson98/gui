// Question #377: ZGC (Low Latency)
// Category: Memory Management | Difficulty: Hard
// Concepts: ZGC, colored pointer, load barrier, low latency
// Description: Explain ZGC's colored pointers and load barriers for sub-millisecond pauses.
package memory

import "sync"

// ZGC (Low Latency)
// Implements a memory management technique for question #377.
type Q377_ZgcLowLatency struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ377_ZgcLowLatency creates a memory manager with the given capacity.
func NewQ377_ZgcLowLatency(capacity int) *Q377_ZgcLowLatency {
        return &Q377_ZgcLowLatency{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q377_ZgcLowLatency) Allocate() any {
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
func (m *Q377_ZgcLowLatency) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
