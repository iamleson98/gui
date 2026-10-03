package concurrency

import (
	"sync"
	"testing"
)

func TestSemaphoreBasic(t *testing.T) {
	s := NewSemaphore(3)
	s.Acquire()
	s.Acquire()
	s.Acquire()
	if s.TryAcquire() {
		t.Fatal("should be full")
	}
	s.Release()
	if !s.TryAcquire() {
		t.Fatal("should have room after release")
	}
}

func TestSemaphoreConcurrency(t *testing.T) {
	s := NewSemaphore(4)
	var wg sync.WaitGroup
	var active int
	var maxActive int
	var mu sync.Mutex
	for i := 0; i < 20; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			s.Acquire()
			mu.Lock()
			active++
			if active > maxActive {
				maxActive = active
			}
			mu.Unlock()
			mu.Lock()
			active--
			mu.Unlock()
			s.Release()
		}()
	}
	wg.Wait()
	if maxActive > 4 {
		t.Fatalf("max active %d exceeded limit 4", maxActive)
	}
}

func BenchmarkSemaphore(b *testing.B) {
	s := NewSemaphore(1)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		s.Acquire()
		s.Release()
	}
}
