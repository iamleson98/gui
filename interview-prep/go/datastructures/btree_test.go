package datastructures

import (
	"math/rand"
	"testing"
)

func TestBTreeBasic(t *testing.T) {
	bt := NewBTree()
	for i := 1; i <= 100; i++ {
		bt.Insert(i)
	}
	for i := 1; i <= 100; i++ {
		if !bt.Search(i) {
			t.Fatalf("should find %d", i)
		}
	}
	if bt.Search(101) {
		t.Fatal("should not find 101")
	}
}

func TestBTreeRandom(t *testing.T) {
	rand.Seed(42)
	bt := NewBTree()
	values := rand.Perm(1000)
	for _, v := range values {
		bt.Insert(v + 1)
	}
	for _, v := range values {
		if !bt.Search(v + 1) {
			t.Fatalf("should find %d", v+1)
		}
	}
}

func TestBTreeRangeQuery(t *testing.T) {
	bt := NewBTree()
	for i := 1; i <= 100; i++ {
		bt.Insert(i)
	}
	result := bt.RangeQuery(10, 20)
	if len(result) != 11 {
		t.Fatalf("expected 11 results, got %d", len(result))
	}
}

func BenchmarkBTreeInsert(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bt := NewBTree()
		for j := 0; j < 1000; j++ {
			bt.Insert(j)
		}
	}
}

func BenchmarkBTreeSearch(b *testing.B) {
	bt := NewBTree()
	for i := 0; i < 10000; i++ {
		bt.Insert(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bt.Search(i % 10000)
	}
}
