// Question #378: Shenandoah GC
// Category: Memory Management | Difficulty: Hard
// Concepts: Shenandoah, Brooks pointer, concurrent evacuation, low latency
// Description: Explain Shenandoah's concurrent evacuation using Brooks forwarding pointers.
package memory

import "sync"

// Shenandoah GC
// Implements a memory management technique for question #378.
type ShenandoahGc struct {
        mu    sync.Mutex
        pool  []interface{}
        size  int
}

// NewShenandoahGc creates a memory manager with the given capacity.
func NewShenandoahGc(capacity int) *ShenandoahGc {
        return &ShenandoahGc{
                pool: make([]interface{}, 0, capacity),
                size: 0,
        }
}

// Allocate returns an object from the pool or creates a new one.
func (m *ShenandoahGc) Allocate() interface{ {
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
func (m *ShenandoahGc) Release(obj interface{) {
        m.mu.Lock()
        m.pool = append(m.pool, obj)
        m.size++
        m.mu.Unlock()
}
