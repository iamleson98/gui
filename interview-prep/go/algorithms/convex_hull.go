// Convex Hull — Andrew's monotone chain in O(n log n).
package algorithms

import "sort"

type Point struct {
	X, Y float64
}

func Cross(O, A, B Point) float64 {
	return (A.X-O.X)*(B.Y-O.Y) - (A.Y-O.Y)*(B.X-O.X)
}

func ConvexHull(points []Point) []Point {
	n := len(points)
	if n <= 2 { return points }
	sorted := make([]Point, n)
	copy(sorted, points)
	sort.Slice(sorted, func(i, j int) bool {
		if sorted[i].X != sorted[j].X { return sorted[i].X < sorted[j].X }
		return sorted[i].Y < sorted[j].Y
	})
	lower := []Point{}
	for _, p := range sorted {
		for len(lower) >= 2 && Cross(lower[len(lower)-2], lower[len(lower)-1], p) <= 0 {
			lower = lower[:len(lower)-1]
		}
		lower = append(lower, p)
	}
	upper := []Point{}
	for i := n - 1; i >= 0; i-- {
		p := sorted[i]
		for len(upper) >= 2 && Cross(upper[len(upper)-2], upper[len(upper)-1], p) <= 0 {
			upper = upper[:len(upper)-1]
		}
		upper = append(upper, p)
	}
	return append(lower[:len(lower)-1], upper[:len(upper)-1]...)
}
