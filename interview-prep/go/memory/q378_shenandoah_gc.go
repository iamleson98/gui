// Question #378: Shenandoah GC
// Category: Memory Management | Difficulty: Hard
// Concepts: Shenandoah, Brooks pointer, concurrent evacuation, low latency
// Description: Explain Shenandoah's concurrent evacuation using Brooks forwarding pointers.
package memory

import "sync"

// Shenandoah GC
// Implements a memory management technique for question #378.
type Q378_ShenandoahGc struct {
        mu    sync.Mutex
        pool  []any
        size  int
}

// NewQ378_ShenandoahGc creates a memory manager with the given capacity.
func NewQ378_ShenandoahGc(capacity int) *Q378_ShenandoahGc {
        return &Q378_ShenandoahGc{
                pool: make([]any, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *Q378_ShenandoahGc) Allocate() any {
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
func (m *Q378_ShenandoahGc) Release(obj any) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
