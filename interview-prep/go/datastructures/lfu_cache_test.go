package datastructures

import "testing"

func TestLFUBasic(t *testing.T) {
	c := NewLFUCache[int, string](2)
	c.Put(1, "a")
	c.Put(2, "b")
	c.Get(1)
	c.Get(2)
	c.Get(1)
	c.Put(3, "c")
	if _, ok := c.Get(2); ok {
		t.Fatal("2 should be evicted")
	}
	v, _ := c.Get(1)
	if v != "a" {
		t.Fatalf("expected a, got %v", v)
	}
}

func TestLFUEvictionByFrequency(t *testing.T) {
	c := NewLFUCache[int, int](3)
	c.Put(1, 1)
	c.Put(2, 2)
	c.Put(3, 3)
	c.Get(1)
	c.Get(1)
	c.Get(2)
	c.Put(4, 4)
	if _, ok := c.Get(3); ok {
		t.Fatal("3 should be evicted (lowest freq)")
	}
}
