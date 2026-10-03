package datastructures

import (
	"fmt"
	"math/rand"
	"testing"
)

func TestSkipListBasic(t *testing.T) {
	sl := NewSkipList[int, string](nil)
	sl.Insert(1, "one")
	sl.Insert(2, "two")
	sl.Insert(3, "three")
	v, ok := sl.Search(2)
	if !ok || v != "two" {
		t.Fatalf("expected two, got %v ok=%v", v, ok)
	}
	if _, ok := sl.Search(99); ok {
		t.Fatal("should not find 99")
	}
	if sl.Len() != 3 {
		t.Fatalf("expected len 3, got %d", sl.Len())
	}
}

func TestSkipListDelete(t *testing.T) {
	sl := NewSkipList[int, int](nil)
	for i := 1; i <= 10; i++ {
		sl.Insert(i, i*10)
	}
	if !sl.Delete(5) {
		t.Fatal("delete 5 failed")
	}
	if _, ok := sl.Search(5); ok {
		t.Fatal("5 should be gone")
	}
	if sl.Len() != 9 {
		t.Fatalf("expected len 9, got %d", sl.Len())
	}
}

func TestSkipListRandom(t *testing.T) {
	rand.Seed(42)
	sl := NewSkipList[int, int](nil)
	N := 10000
	for i := 0; i < N; i++ {
		sl.Insert(rand.Intn(N), 1)
	}
	for i := 0; i < N; i++ {
		sl.Search(i)
	}
}

func TestSkipListStrings(t *testing.T) {
	sl := NewSkipList[string, int](nil)
	sl.Insert("banana", 1)
	sl.Insert("apple", 2)
	sl.Insert("cherry", 3)
	v, ok := sl.Search("apple")
	if !ok || v != 2 {
		t.Fatalf("expected apple=2, got %v", v)
	}
}

func BenchmarkSkipListInsert(b *testing.B) {
	sl := NewSkipList[int, int](nil)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		sl.Insert(i, i)
	}
}

func BenchmarkSkipListSearch(b *testing.B) {
	sl := NewSkipList[int, int](nil)
	for i := 0; i < 10000; i++ {
		sl.Insert(i, i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		sl.Search(i % 10000)
	}
}

func ExampleSkipList() {
	sl := NewSkipList[int, string](nil)
	sl.Insert(1, "hello")
	v, _ := sl.Search(1)
	fmt.Println(v)
	// Output: hello
}
