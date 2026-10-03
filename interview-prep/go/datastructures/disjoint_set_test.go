package datastructures

import "testing"

func TestDisjointSetBasic(t *testing.T) {
	d := NewDisjointSet(10)
	d.Union(0, 1)
	d.Union(2, 3)
	d.Union(1, 3)
	if !d.Connected(0, 2) {
		t.Fatal("0 and 2 should be connected")
	}
	if d.Connected(0, 4) {
		t.Fatal("0 and 4 should not be connected")
	}
}

func TestDisjointSetAllUnion(t *testing.T) {
	d := NewDisjointSet(100)
	for i := 0; i < 99; i++ {
		d.Union(i, i+1)
	}
	root := d.Find(0)
	for i := 1; i < 100; i++ {
		if d.Find(i) != root {
			t.Fatalf("node %d has different root", i)
		}
	}
}

func TestDisjointSetRedundantUnion(t *testing.T) {
	d := NewDisjointSet(5)
	if !d.Union(0, 1) {
		t.Fatal("first union should return true")
	}
	if d.Union(0, 1) {
		t.Fatal("second union should return false")
	}
}

func BenchmarkDisjointSet(b *testing.B) {
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		d := NewDisjointSet(10000)
		for j := 0; j < 5000; j++ {
			d.Union(j, j+5000)
		}
		for j := 0; j < 10000; j++ {
			d.Find(j)
		}
	}
}
