package concurrency

import "testing"

func TestSPSCRingBasic(t *testing.T) {
	r := NewSPSCRing[int](8)
	for i := 0; i < 8; i++ {
		if !r.TryEnqueue(i) {
			t.Fatalf("enqueue %d failed", i)
		}
	}
	if r.TryEnqueue(99) {
		t.Fatal("enqueue on full should fail")
	}
	for i := 0; i < 8; i++ {
		v, ok := r.TryDequeue()
		if !ok || v != i {
			t.Fatalf("dequeue %d: expected %d, got %v ok=%v", i, i, v, ok)
		}
	}
	if _, ok := r.TryDequeue(); ok {
		t.Fatal("dequeue on empty should return false")
	}
}

func TestSPSCRingInterleaved(t *testing.T) {
	r := NewSPSCRing[int](4)
	r.TryEnqueue(1)
	r.TryEnqueue(2)
	v, _ := r.TryDequeue()
	if v != 1 {
		t.Fatalf("expected 1, got %v", v)
	}
	r.TryEnqueue(3)
	r.TryEnqueue(4)
	r.TryEnqueue(5)
	for _, exp := range []int{2, 3, 4} {
		v, _ := r.TryDequeue()
		if v != exp {
			t.Fatalf("expected %d, got %v", exp, v)
		}
	}
}

func BenchmarkSPSCRingEnqueue(b *testing.B) {
	r := NewSPSCRing[int](1024)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		r.TryEnqueue(i)
		if r.Len() > 0 {
			r.TryDequeue()
		}
	}
}
