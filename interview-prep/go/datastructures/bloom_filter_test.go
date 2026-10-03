package datastructures

import (
	"fmt"
	"testing"
)

func TestBloomFilterNoFalseNegatives(t *testing.T) {
	bf := NewBloomFilter(1000, 0.01)
	for i := 0; i < 1000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	for i := 0; i < 1000; i++ {
		if !bf.ContainsString(fmt.Sprintf("item-%d", i)) {
			t.Fatalf("false negative for item-%d", i)
		}
	}
}

func TestBloomFilterFalsePositiveRate(t *testing.T) {
	bf := NewBloomFilter(10000, 0.01)
	for i := 0; i < 10000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	fp := 0
	for i := 10000; i < 20000; i++ {
		if bf.ContainsString(fmt.Sprintf("item-%d", i)) {
			fp++
		}
	}
	if float64(fp)/10000 > 0.05 {
		t.Fatalf("false positive rate too high: fp=%d", fp)
	}
}

func BenchmarkBloomFilterAdd(b *testing.B) {
	bf := NewBloomFilter(100000, 0.01)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
}

func BenchmarkBloomFilterContains(b *testing.B) {
	bf := NewBloomFilter(100000, 0.01)
	for i := 0; i < 100000; i++ {
		bf.AddString(fmt.Sprintf("item-%d", i))
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		bf.ContainsString(fmt.Sprintf("item-%d", i%100000))
	}
}
