// Readers-Writer Lock (reader preference).
package concurrency

import "sync"

type RWLock struct {
	mu      sync.Mutex
	readers int
	writer  bool
	cond    *sync.Cond
}

func NewRWLock() *RWLock {
	l := &RWLock{}
	l.cond = sync.NewCond(&l.mu)
	return l
}

func (l *RWLock) RLock() {
	l.mu.Lock()
	for l.writer {
		l.cond.Wait()
	}
	l.readers++
	l.mu.Unlock()
}

func (l *RWLock) RUnlock() {
	l.mu.Lock()
	l.readers--
	if l.readers == 0 {
		l.cond.Broadcast()
	}
	l.mu.Unlock()
}

func (l *RWLock) Lock() {
	l.mu.Lock()
	for l.writer || l.readers > 0 {
		l.cond.Wait()
	}
	l.writer = true
	l.mu.Unlock()
}

func (l *RWLock) Unlock() {
	l.mu.Lock()
	l.writer = false
	l.cond.Broadcast()
	l.mu.Unlock()
}
