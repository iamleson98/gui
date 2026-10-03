// Dijkstra's Algorithm — single-source shortest path.
package algorithms

import "container/heap"

type edge struct {
	to   int
	cost int64
}

type Graph struct {
	n   int
	adj [][]edge
}

func NewGraph(n int) *Graph {
	return &Graph{n: n, adj: make([][]edge, n)}
}

func (g *Graph) AddEdge(from, to int, cost int64) {
	g.adj[from] = append(g.adj[from], edge{to, cost})
}

type pqItem struct {
	node int
	dist int64
	idx  int
}

type priorityQueue []*pqItem

func (pq priorityQueue) Len() int            { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool  { return pq[i].dist < pq[j].dist }
func (pq priorityQueue) Swap(i, j int)       { pq[i], pq[j] = pq[j], pq[i]; pq[i].idx = i; pq[j].idx = j }
func (pq *priorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*pqItem)
	item.idx = n
	*pq = append(*pq, item)
}
func (pq *priorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[:n-1]
	return item
}

func Dijkstra(g *Graph, source int) []int64 {
	dist := make([]int64, g.n)
	for i := range dist { dist[i] = -1 }
	dist[source] = 0
	pq := &priorityQueue{}
	heap.Init(pq)
	heap.Push(pq, &pqItem{node: source, dist: 0})
	visited := make([]bool, g.n)
	for pq.Len() > 0 {
		item := heap.Pop(pq).(*pqItem)
		u := item.node
		if visited[u] { continue }
		visited[u] = true
		for _, e := range g.adj[u] {
			v := e.to
			newDist := dist[u] + e.cost
			if dist[v] == -1 || newDist < dist[v] {
				dist[v] = newDist
				heap.Push(pq, &pqItem{node: v, dist: newDist})
			}
		}
	}
	return dist
}
