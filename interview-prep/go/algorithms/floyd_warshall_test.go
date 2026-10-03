package algorithms

import "testing"

func TestFloydWarshall(t *testing.T) {
	INF := FW_INF
	dist := [][]int64{
		{0, 3, INF, 5},
		{2, 0, INF, 4},
		{INF, 1, 0, INF},
		{INF, INF, 2, 0},
	}
	result := FloydWarshall(dist)
	expected := [][]int64{
		{0, 3, 7, 5},
		{2, 0, 6, 4},
		{3, 1, 0, 5},
		{5, 3, 2, 0},
	}
	for i := range result {
		for j := range result[i] {
			if result[i][j] != expected[i][j] {
				t.Fatalf("result[%d][%d] = %d, expected %d", i, j, result[i][j], expected[i][j])
			}
		}
	}
}

func TestFloydWarshallNegativeCycle(t *testing.T) {
	INF := FW_INF
	dist := [][]int64{
		{0, 1, INF},
		{INF, 0, -1},
		{-1, INF, 0},
	}
	if !HasNegativeCycle(dist) {
		t.Fatal("should detect negative cycle")
	}
}

func BenchmarkFloydWarshall(b *testing.B) {
	n := 100
	dist := make([][]int64, n)
	for i := range dist {
		dist[i] = make([]int64, n)
		for j := range dist[i] {
			if i == j { dist[i][j] = 0 } else { dist[i][j] = FW_INF }
		}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		FloydWarshall(dist)
	}
}
