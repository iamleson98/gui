// Question #383: Tracing vs Reference Counting
// Category: Memory Management | Difficulty: Hard
// Concepts: tracing, reference counting, pause, throughput
// Description: Contrast tracing and reference counting collectors on pause time and throughput.
package memory

import "sync"

// Tracing vs Reference Counting
// Implements a memory management technique for question #383.
type TracingVsReferenceCounting struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewTracingVsReferenceCounting creates a memory manager with the given capacity.
func NewTracingVsReferenceCounting(capacity int) *TracingVsReferenceCounting {
        return &TracingVsReferenceCounting{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *TracingVsReferenceCounting) Allocate() interface{ {
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
func (m *TracingVsReferenceCounting) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
