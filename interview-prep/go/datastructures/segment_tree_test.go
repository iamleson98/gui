package datastructures

import "testing"

func TestSegmentTreeBasic(t *testing.T) {
	arr := []int64{1, 3, 5, 7, 9, 11}
	st := NewSegmentTree(arr)
	if v := st.QueryRange(0, 5); v != 36 {
		t.Fatalf("expected 36, got %d", v)
	}
	if v := st.QueryRange(1, 3); v != 15 {
		t.Fatalf("expected 15, got %d", v)
	}
}

func TestSegmentTreeRangeUpdate(t *testing.T) {
	arr := []int64{1, 2, 3, 4, 5}
	st := NewSegmentTree(arr)
	st.UpdateRange(0, 2, 10)
	if v := st.QueryRange(0, 2); v != 36 {
		t.Fatalf("expected 36, got %d", v)
	}
	if v := st.QueryRange(3, 4); v != 9 {
		t.Fatalf("expected 9, got %d", v)
	}
}

func BenchmarkSegmentTreeUpdate(b *testing.B) {
	arr := make([]int64, 100000)
	st := NewSegmentTree(arr)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		st.UpdateRange(0, 99999, 1)
	}
}

func BenchmarkSegmentTreeQuery(b *testing.B) {
	arr := make([]int64, 100000)
	st := NewSegmentTree(arr)
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		st.QueryRange(0, 99999)
	}
}
