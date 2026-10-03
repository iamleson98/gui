package algorithms

import "testing"

func TestDijkstraBasic(t *testing.T) {
	g := NewGraph(5)
	g.AddEdge(0, 1, 10)
	g.AddEdge(0, 4, 5)
	g.AddEdge(1, 2, 1)
	g.AddEdge(1, 4, 2)
	g.AddEdge(2, 3, 4)
	g.AddEdge(3, 0, 7)
	g.AddEdge(4, 1, 3)
	g.AddEdge(4, 2, 9)
	g.AddEdge(4, 3, 2)
	dist := Dijkstra(g, 0)
	expected := []int64{0, 8, 9, 7, 5}
	for i, d := range dist {
		if d != expected[i] {
			t.Fatalf("dist[%d] = %d, expected %d", i, d, expected[i])
		}
	}
}

func TestDijkstraUnreachable(t *testing.T) {
	g := NewGraph(4)
	g.AddEdge(0, 1, 1)
	g.AddEdge(1, 2, 1)
	dist := Dijkstra(g, 0)
	if dist[3] != -1 {
		t.Fatalf("dist[3] should be -1, got %d", dist[3])
	}
}

func BenchmarkDijkstra(b *testing.B) {
	n := 1000
	g := NewGraph(n)
	for i := 0; i < n; i++ {
		for j := 1; j <= 5; j++ {
			g.AddEdge(i, (i+j)%n, int64(j))
		}
	}
	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Dijkstra(g, 0)
	}
}
