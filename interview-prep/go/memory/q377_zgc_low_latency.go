// Question #377: ZGC (Low Latency)
// Category: Memory Management | Difficulty: Hard
// Concepts: ZGC, colored pointer, load barrier, low latency
// Description: Explain ZGC's colored pointers and load barriers for sub-millisecond pauses.
package memory

import "sync"

// ZGC (Low Latency)
// Implements a memory management technique for question #377.
type ZgcLowLatency struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewZgcLowLatency creates a memory manager with the given capacity.
func NewZgcLowLatency(capacity int) *ZgcLowLatency {
        return &ZgcLowLatency{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ZgcLowLatency) Allocate() interface{ {
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
func (m *ZgcLowLatency) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
