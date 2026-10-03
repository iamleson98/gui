package algorithms

import "testing"

func TestKnapsack01(t *testing.T) {
	items := []Item{
		{Weight: 2, Value: 3},
		{Weight: 3, Value: 4},
		{Weight: 4, Value: 5},
		{Weight: 5, Value: 6},
	}
	result := Knapsack01(items, 5)
	if result != 7 {
		t.Fatalf("expected 7, got %d", result)
	}
}

func TestKnapsackEmpty(t *testing.T) {
	if Knapsack01(nil, 10) != 0 {
		t.Fatal("empty knapsack should be 0")
	}
}

func TestKnapsackNoFit(t *testing.T) {
	items := []Item{{Weight: 100, Value: 1000}}
	if Knapsack01(items, 10) != 0 {
		t.Fatal("should be 0 when nothing fits")
	}
}

func BenchmarkKnapsack(b *testing.B) {
	items := make([]Item, 100)
	for i := range items {
		items[i] = Item{Weight: i + 1, Value: i*2 + 1}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Knapsack01(items, 5000)
	}
}
