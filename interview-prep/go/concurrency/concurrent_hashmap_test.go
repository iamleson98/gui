package concurrency

import (
	"fmt"
	"sync"
	"testing"
)

func TestConcurrentMapBasic(t *testing.T) {
	m := NewConcurrentMap[string, int](32)
	m.Put("a", 1)
	m.Put("b", 2)
	m.Put("c", 3)
	v, ok := m.Get("a")
	if !ok || v != 1 {
		t.Fatalf("expected a=1, got %v ok=%v", v, ok)
	}
	m.Delete("b")
	if _, ok := m.Get("b"); ok {
		t.Fatal("b should be deleted")
	}
	if m.Len() != 2 {
		t.Fatalf("expected len 2, got %d", m.Len())
	}
}

func TestConcurrentMapConcurrent(t *testing.T) {
	m := NewConcurrentMap[string, int](64)
	var wg sync.WaitGroup
	N := 8
	M := 1000
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			for j := 0; j < M; j++ {
				key := fmt.Sprintf("k-%d-%d", id, j)
				m.Put(key, j)
			}
		}(i)
	}
	wg.Wait()
	if m.Len() != N*M {
		t.Fatalf("expected %d, got %d", N*M, m.Len())
	}
	for i := 0; i < N; i++ {
		for j := 0; j < M; j++ {
			key := fmt.Sprintf("k-%d-%d", i, j)
			v, ok := m.Get(key)
			if !ok || v != j {
				t.Fatalf("missing %s", key)
			}
		}
	}
}

func BenchmarkConcurrentMapPut(b *testing.B) {
	m := NewConcurrentMap[string, int](64)
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			m.Put(fmt.Sprintf("k-%d", i), i)
			i++
		}
	})
}

func BenchmarkConcurrentMapGet(b *testing.B) {
	m := NewConcurrentMap[string, int](64)
	for i := 0; i < 10000; i++ {
		m.Put(fmt.Sprintf("k-%d", i), i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			m.Get(fmt.Sprintf("k-%d", i%10000))
			i++
		}
	})
}
