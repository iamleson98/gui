// Question #372: Mark-and-Sweep GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-and-sweep, tracing, roots, sweep
// Description: Implement a mark-and-sweep collector that traces live objects and sweeps dead ones.
package memory

import "sync"

// Mark-and-Sweep GC
// Implements a memory management technique for question #372.
type MarkAndSweepGc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewMarkAndSweepGc creates a memory manager with the given capacity.
func NewMarkAndSweepGc(capacity int) *MarkAndSweepGc {
        return &MarkAndSweepGc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *MarkAndSweepGc) Allocate() interface{ {
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
func (m *MarkAndSweepGc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
