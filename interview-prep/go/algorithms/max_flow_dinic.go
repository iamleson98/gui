// Max Flow — Dinic's algorithm.
package algorithms

type flowEdge struct {
	to   int
	cap  int64
	rev  int
}

type MaxFlow struct {
	n     int
	graph [][]flowEdge
}

func NewMaxFlow(n int) *MaxFlow {
	return &MaxFlow{n: n, graph: make([][]flowEdge, n)}
}

func (mf *MaxFlow) AddEdge(from, to int, cap int64) {
	mf.graph[from] = append(mf.graph[from], flowEdge{to: to, cap: cap, rev: len(mf.graph[to])})
	mf.graph[to] = append(mf.graph[to], flowEdge{to: from, cap: 0, rev: len(mf.graph[from]) - 1})
}

func (mf *MaxFlow) bfs(s, t int, level []int) bool {
	for i := range level { level[i] = -1 }
	level[s] = 0
	queue := []int{s}
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for _, e := range mf.graph[u] {
			if e.cap > 0 && level[e.to] < 0 {
				level[e.to] = level[u] + 1
				queue = append(queue, e.to)
			}
		}
	}
	return level[t] >= 0
}

func (mf *MaxFlow) dfs(u, t int, f int64, level []int, iter []int) int64 {
	if u == t { return f }
	for ; iter[u] < len(mf.graph[u]); iter[u]++ {
		e := &mf.graph[u][iter[u]]
		if e.cap > 0 && level[e.to] == level[u]+1 {
			d := mf.dfs(e.to, t, min64(f, e.cap), level, iter)
			if d > 0 {
				e.cap -= d
				mf.graph[e.to][e.rev].cap += d
				return d
			}
		}
	}
	return 0
}

func (mf *MaxFlow) MaxFlow(s, t int) int64 {
	flow := int64(0)
	level := make([]int, mf.n)
	iter := make([]int, mf.n)
	const INF int64 = 1 << 60
	for mf.bfs(s, t, level) {
		for i := range iter { iter[i] = 0 }
		for {
			f := mf.dfs(s, t, INF, level, iter)
			if f == 0 { break }
			flow += f
		}
	}
	return flow
}

func min64(a, b int64) int64 {
	if a < b { return a }
	return b
}
