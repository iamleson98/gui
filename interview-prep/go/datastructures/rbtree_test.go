package datastructures

import (
	"math/rand"
	"testing"
)

func TestRBTreeBasic(t *testing.T) {
	tree := NewRBTree()
	for i := 1; i <= 100; i++ {
		tree.Insert(i)
	}
	for i := 1; i <= 100; i++ {
		if !tree.Search(i) {
			t.Fatalf("should find %d", i)
		}
	}
}

func TestRBTreeInvariants(t *testing.T) {
	rand.Seed(42)
	tree := NewRBTree()
	for i := 0; i < 1000; i++ {
		tree.Insert(rand.Intn(10000))
	}
	if err := tree.VerifyInvariants(); err != nil {
		t.Fatalf("invariant violation: %v", err)
	}
}

func TestRBTreeInorder(t *testing.T) {
	tree := NewRBTree()
	values := []int{5, 3, 7, 1, 4, 6, 8, 2}
	for _, v := range values {
		tree.Insert(v)
	}
	sorted := tree.Inorder()
	expected := []int{1, 2, 3, 4, 5, 6, 7, 8}
	for i, v := range sorted {
		if v != expected[i] {
			t.Fatalf("at index %d: expected %d, got %d", i, expected[i], v)
		}
	}
}

func BenchmarkRBTreeInsert(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tree := NewRBTree()
		for j := 0; j < 1000; j++ {
			tree.Insert(j)
		}
	}
}

func BenchmarkRBTreeSearch(b *testing.B) {
	tree := NewRBTree()
	for i := 0; i < 10000; i++ {
		tree.Insert(i)
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		tree.Search(i % 10000)
	}
}
