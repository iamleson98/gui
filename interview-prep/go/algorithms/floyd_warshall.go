// Floyd-Warshall — all-pairs shortest paths in O(V^3).
package algorithms

const FW_INF int64 = 1 << 60

func FloydWarshall(dist [][]int64) [][]int64 {
	n := len(dist)
	result := make([][]int64, n)
	for i := range result {
		result[i] = make([]int64, n)
		copy(result[i], dist[i])
	}
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				if result[i][k] != FW_INF && result[k][j] != FW_INF {
					if result[i][k]+result[k][j] < result[i][j] {
						result[i][j] = result[i][k] + result[k][j]
					}
				}
			}
		}
	}
	return result
}

func HasNegativeCycle(dist [][]int64) bool {
	n := len(dist)
	result := FloydWarshall(dist)
	for i := 0; i < n; i++ {
		if result[i][i] < 0 {
			return true
		}
	}
	return false
}
