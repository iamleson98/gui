package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestTreiberStackPushPop(t *testing.T) {
	s := NewTreiberStack[int]()
	if !s.Empty() {
		t.Fatal("new stack should be empty")
	}
	s.Push(1)
	s.Push(2)
	s.Push(3)
	if s.Empty() {
		t.Fatal("stack should not be empty after pushes")
	}
	v, ok := s.Pop()
	if !ok || v != 3 {
		t.Fatalf("expected 3, got %v (ok=%v)", v, ok)
	}
	v, _ = s.Pop()
	if v != 2 {
		t.Fatalf("expected 2, got %v", v)
	}
	v, _ = s.Pop()
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	if _, ok := s.Pop(); ok {
		t.Fatal("pop on empty should return ok=false")
	}
	if !s.Empty() {
		t.Fatal("stack should be empty after draining")
	}
}

func TestTreiberStackConcurrent(t *testing.T) {
	s := NewTreiberStack[int]()
	var wg sync.WaitGroup
	// 8 producers, each pushing 1000 values
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < 1000; j++ {
				s.Push(id*1000 + j)
			}
		}(i)
	}
	wg.Wait()

	// Count total elements
	count := 0
	for {
		_, ok := s.Pop()
		if !ok {
			break
		}
		count++
	}
	if count != 8000 {
		t.Fatalf("expected 8000 elements, got %d", count)
	}
}

func TestTreiberStackConcurrentMixed(t *testing.T) {
	s := NewTreiberStack[int]()
	var wg sync.WaitGroup
	// 4 pushers + 4 poppers concurrently
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < 500; j++ {
				s.Push(id*500 + j)
			}
		}(i)
	}
	var popped int64
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < 500; j++ {
				if _, ok := s.Pop(); ok {
					atomic.AddInt64(&popped, 1)
				}
			}
		}()
	}
	wg.Wait()
	// Remaining elements
	remaining := 0
	for {
		_, ok := s.Pop()
		if !ok {
			break
		}
		remaining++
	}
	total := int(popped) + remaining
	if total != 2000 {
		t.Fatalf("expected 2000 total, got %d (popped=%d, remaining=%d)", total, popped, remaining)
	}
}

func BenchmarkTreiberStackPush(b *testing.B) {
	s := NewTreiberStack[int]()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		s.Push(i)
	}
}

func BenchmarkTreiberStackPop(b *testing.B) {
	s := NewTreiberStack[int]()
	for i := 0; i < b.N; i++ {
		s.Push(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		s.Pop()
	}
}

func BenchmarkTreiberStackConcurrentPush(b *testing.B) {
	s := NewTreiberStack[int]()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			s.Push(i)
			i++
		}
	})
}
