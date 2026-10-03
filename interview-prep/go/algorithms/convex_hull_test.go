package algorithms

import (
	"math"
	"testing"
)

func pointsEqual(a, b Point) bool {
	return math.Abs(a.X-b.X) < 1e-9 && math.Abs(a.Y-b.Y) < 1e-9
}

func TestConvexHullBasic(t *testing.T) {
	points := []Point{
		{0, 0}, {1, 0}, {0, 1}, {1, 1},
		{0.5, 0.5},
	}
	hull := ConvexHull(points)
	if len(hull) != 4 {
		t.Fatalf("expected 4 hull points, got %d: %v", len(hull), hull)
	}
}

func TestConvexHullCollinear(t *testing.T) {
	points := []Point{
		{0, 0}, {1, 0}, {2, 0}, {3, 0},
	}
	hull := ConvexHull(points)
	if len(hull) < 2 {
		t.Fatalf("expected at least 2 hull points, got %d", len(hull))
	}
}

func BenchmarkConvexHull(b *testing.B) {
	points := make([]Point, 10000)
	for i := range points {
		points[i] = Point{X: float64(i % 100), Y: float64(i / 100)}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		ConvexHull(points)
	}
}
