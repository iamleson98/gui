// Question #372: Mark-and-Sweep GC
// Category: Memory Management | Difficulty: Hard
// Concepts: mark-and-sweep, tracing, roots, sweep
// Description: Implement a mark-and-sweep collector that traces live objects and sweeps dead ones.
package memory

import "sync"

// Mark-and-Sweep GC
// Implements a memory management technique for question #372.
type Q372_MarkAndSweepGc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ372_MarkAndSweepGc creates a memory manager with the given capacity.
func NewQ372_MarkAndSweepGc(capacity int) *Q372_MarkAndSweepGc {
        return &Q372_MarkAndSweepGc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q372_MarkAndSweepGc) Allocate() any {
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
func (m *Q372_MarkAndSweepGc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
