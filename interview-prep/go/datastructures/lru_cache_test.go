package datastructures

import "testing"

func TestLRUBasic(t *testing.T) {
	c := NewLRUCache[int, string](2)
	c.Put(1, "a")
	c.Put(2, "b")
	v, ok := c.Get(1)
	if !ok || v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
	c.Put(3, "c")
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should have been evicted")
	}
	v, _ = c.Get(1)
	if v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
}

func TestLRUUpdate(t *testing.T) {
	c := NewLRUCache[int, int](2)
	c.Put(1, 10)
	c.Put(1, 20)
	v, _ := c.Get(1)
	if v != 20 {
		t.Fatalf("expected 20, got %d", v)
	}
	if c.Len() != 1 {
		t.Fatalf("expected len 1, got %d", c.Len())
	}
}

func TestLRUEvictionOrder(t *testing.T) {
	c := NewLRUCache[int, int](3)
	c.Put(1, 1)
	c.Put(2, 2)
	c.Put(3, 3)
	c.Get(1)
	c.Put(4, 4)
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should be evicted")
	}
	for _, k := range []int{1, 3, 4} {
		if _, ok := c.Get(k); !ok {
			t.Fatalf("%d should be present", k)
		}
	}
}

func BenchmarkLRUPut(b *testing.B) {
	c := NewLRUCache[int, int](1000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		c.Put(i, i)
	}
}

func BenchmarkLRUGet(b *testing.B) {
	c := NewLRUCache[int, int](1000)
	for i := 0; i < 1000; i++ {
		c.Put(i, i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		c.Get(i % 1000)
	}
}
