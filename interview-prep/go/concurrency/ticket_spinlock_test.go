package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestTicketSpinlockBasic(t *testing.T) {
	var l TicketSpinlock
	l.Lock()
	if l.TryLock() {
		t.Fatal("TryLock should fail when locked")
	}
	l.Unlock()
	if !l.TryLock() {
		t.Fatal("TryLock should succeed when unlocked")
	}
	l.Unlock()
}

func TestTicketSpinlockMutualExclusion(t *testing.T) {
	var l TicketSpinlock
	var counter int64
	var wg sync.WaitGroup
	N := 8
	M := 10000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < M; j++ {
				l.Lock()
				atomic.AddInt64(&counter, 1)
				l.Unlock()
			}
		}()
	}
	wg.Wait()
	expected := int64(N * M)
	if counter != expected {
		t.Fatalf("expected %d, got %d", expected, counter)
	}
}

func BenchmarkTicketSpinlock(b *testing.B) {
	var l TicketSpinlock
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		l.Lock()
		l.Unlock()
	}
}
