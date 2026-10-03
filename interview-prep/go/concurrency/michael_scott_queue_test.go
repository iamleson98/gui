package concurrency

import (
	"sync"
	"sync/atomic"
	"testing"
)

func TestMSQueueBasic(t *testing.T) {
	q := NewMSQueue[int]()
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	v, ok := q.Dequeue()
	if !ok || v != 1 {
		t.Fatalf("expected 1, got %v ok=%v", v, ok)
	}
	v, _ = q.Dequeue()
	if v != 2 {
		t.Fatalf("expected 2, got %v", v)
	}
	v, _ = q.Dequeue()
	if v != 3 {
		t.Fatalf("expected 3, got %v", v)
	}
	if _, ok := q.Dequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestMSQueueConcurrent(t *testing.T) {
	q := NewMSQueue[int]()
	var wg sync.WaitGroup
	N, M := 4, 2000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				q.Enqueue(id*M + j)
			}
		}(i)
	}
	var consumed int64
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < M; j++ {
				if _, ok := q.Dequeue(); ok {
					atomic.AddInt64(&consumed, 1)
				}
			}
		}()
	}
	wg.Wait()
	for {
		_, ok := q.Dequeue()
		if !ok {
			break
		}
		atomic.AddInt64(&consumed, 1)
	}
	total := int(consumed)
	expected := N * M
	if total != expected {
		t.Fatalf("expected %d, got %d", expected, total)
	}
}

func BenchmarkMSQueueEnqueue(b *testing.B) {
	q := NewMSQueue[int]()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		q.Enqueue(i)
	}
}

func BenchmarkMSQueueDequeue(b *testing.B) {
	q := NewMSQueue[int]()
	for i := 0; i < b.N; i++ {
		q.Enqueue(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		q.Dequeue()
	}
}

func BenchmarkMSQueueConcurrent(b *testing.B) {
	q := NewMSQueue[int]()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			q.Enqueue(i)
			q.Dequeue()
			i++
		}
	})
}
