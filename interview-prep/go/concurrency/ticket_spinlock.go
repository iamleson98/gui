// Ticket Spinlock — FIFO fair spinlock.
package concurrency

import (
        "runtime"
        "sync/atomic"
)

type TicketSpinlock struct {
        next atomic.Int64
        now  atomic.Int64
}

func (l *TicketSpinlock) Lock() {
        ticket := l.next.Add(1) - 1 // Add returns new value; subtract 1 for old
        for l.now.Load() != ticket {
                runtime.Gosched()
        }
}

func (l *TicketSpinlock) Unlock() {
        l.now.Add(1)
}

func (l *TicketSpinlock) TryLock() bool {
        now := l.now.Load()
        next := l.next.Load()
        if now != next {
                return false // someone is waiting
        }
        // Try to acquire
        if l.next.CompareAndSwap(next, next+1) {
                if l.now.CompareAndSwap(now, now+1) {
                        return true
                }
                // Shouldn't happen in practice
        }
        return false
}
