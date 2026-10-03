// Condition Variable built on sync.Cond.
// Key concepts: condition variable, wait/notify, spurious wakeups.
package concurrency

import "sync"

// CondVar wraps sync.Cond for a simple condition variable interface.
// The caller must hold Lock() before calling WaitLocked/Signal/Broadcast.
type CondVar struct {
	cond *sync.Cond
}

func NewCondVar() *CondVar {
	return &CondVar{cond: sync.NewCond(&sync.Mutex{})}
}

// Wait blocks until pred() returns true. Acquires and releases the lock
// internally — do NOT hold Lock() when calling this.
func (c *CondVar) Wait(pred func() bool) {
	c.cond.L.Lock()
	for !pred() {
		c.cond.Wait()
	}
	c.cond.L.Unlock()
}

// Signal wakes one waiter. Acquires the lock internally.
func (c *CondVar) Signal() {
	c.cond.L.Lock()
	c.cond.Signal()
	c.cond.L.Unlock()
}

// Broadcast wakes all waiters. Acquires the lock internally.
func (c *CondVar) Broadcast() {
	c.cond.L.Lock()
	c.cond.Broadcast()
	c.cond.L.Unlock()
}

func (c *CondVar) Lock()   { c.cond.L.Lock() }
func (c *CondVar) Unlock() { c.cond.L.Unlock() }
