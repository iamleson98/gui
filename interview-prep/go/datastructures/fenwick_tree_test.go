package datastructures

import "testing"

func TestFenwickBasic(t *testing.T) {
	arr := []int64{1, 3, 5, 7, 9, 11}
	ft := NewFenwickTreeFrom(arr)
	if v := ft.Query(2); v != 9 {
		t.Fatalf("expected 9, got %d", v)
	}
	if v := ft.QueryRange(1, 3); v != 15 {
		t.Fatalf("expected 15, got %d", v)
	}
}

func TestFenwickUpdate(t *testing.T) {
	arr := []int64{10, 20, 30, 40, 50}
	ft := NewFenwickTreeFrom(arr)
	ft.Update(2, 5)
	if v := ft.Query(4); v != 155 {
		t.Fatalf("expected 155, got %d", v)
	}
}

func BenchmarkFenwickUpdate(b *testing.B) {
	ft := NewFenwickTree(100000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ft.Update(i%100000, 1)
	}
}

func BenchmarkFenwickQuery(b *testing.B) {
	ft := NewFenwickTree(100000)
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ft.Query(i % 100000)
	}
}
