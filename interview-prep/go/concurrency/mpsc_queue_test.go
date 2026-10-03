package concurrency

import (
	"sync"
	"testing"
)

func TestMPSCQueueBasic(t *testing.T) {
	q := NewMPSCQueue[int]()
	if !q.Empty() {
		t.Fatal("new queue should be empty")
	}
	q.Enqueue(10)
	q.Enqueue(20)
	q.Enqueue(30)
	v, ok := q.Dequeue()
	if !ok || v != 10 {
		t.Fatalf("expected 10, got %v ok=%v", v, ok)
	}
	v, _ = q.Dequeue()
	if v != 20 {
		t.Fatalf("expected 20, got %v", v)
	}
	v, _ = q.Dequeue()
	if v != 30 {
		t.Fatalf("expected 30, got %v", v)
	}
	if _, ok := q.Dequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestMPSCQueueMultiProducer(t *testing.T) {
	q := NewMPSCQueue[int]()
	var wg sync.WaitGroup
	N, M := 8, 500
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				q.Enqueue(id*M + j)
			}
		}(i)
	}
	wg.Wait()
	seen := make(map[int]bool)
	count := 0
	for {
		v, ok := q.Dequeue()
		if !ok {
			break
		}
		if seen[v] {
			t.Fatalf("duplicate value %d", v)
		}
		seen[v] = true
		count++
	}
	expected := N * M
	if count != expected {
		t.Fatalf("expected %d, got %d", expected, count)
	}
}

func BenchmarkMPSCQueueEnqueue(b *testing.B) {
	q := NewMPSCQueue[int]()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			q.Enqueue(i)
			i++
		}
	})
}
