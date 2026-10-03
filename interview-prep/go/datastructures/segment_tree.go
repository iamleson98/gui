// Segment Tree with Lazy Propagation.
package datastructures

type SegmentTree struct {
	n    int
	tree []int64
	lazy []int64
}

func NewSegmentTree(arr []int64) *SegmentTree {
	n := len(arr)
	st := &SegmentTree{n: n, tree: make([]int64, 4*n), lazy: make([]int64, 4*n)}
	if n > 0 {
		st.build(arr, 0, 0, n-1)
	}
	return st
}

func (st *SegmentTree) build(arr []int64, node, start, end int) {
	if start == end {
		st.tree[node] = arr[start]
		return
	}
	mid := (start + end) / 2
	st.build(arr, 2*node+1, start, mid)
	st.build(arr, 2*node+2, mid+1, end)
	st.tree[node] = st.tree[2*node+1] + st.tree[2*node+2]
}

func (st *SegmentTree) pushDown(node, start, end int) {
	if st.lazy[node] != 0 {
		st.tree[node] += int64(end-start+1) * st.lazy[node]
		if start != end {
			st.lazy[2*node+1] += st.lazy[node]
			st.lazy[2*node+2] += st.lazy[node]
		}
		st.lazy[node] = 0
	}
}

func (st *SegmentTree) UpdateRange(l, r int, val int64) {
	if st.n == 0 {
		return
	}
	st.updateRange(0, 0, st.n-1, l, r, val)
}

func (st *SegmentTree) updateRange(node, start, end, l, r int, val int64) {
	st.pushDown(node, start, end)
	if start > r || end < l {
		return
	}
	if l <= start && end <= r {
		st.lazy[node] += val
		st.pushDown(node, start, end)
		return
	}
	mid := (start + end) / 2
	st.updateRange(2*node+1, start, mid, l, r, val)
	st.updateRange(2*node+2, mid+1, end, l, r, val)
	st.tree[node] = st.tree[2*node+1] + st.tree[2*node+2]
}

func (st *SegmentTree) QueryRange(l, r int) int64 {
	if st.n == 0 {
		return 0
	}
	return st.queryRange(0, 0, st.n-1, l, r)
}

func (st *SegmentTree) queryRange(node, start, end, l, r int) int64 {
	st.pushDown(node, start, end)
	if start > r || end < l {
		return 0
	}
	if l <= start && end <= r {
		return st.tree[node]
	}
	mid := (start + end) / 2
	return st.queryRange(2*node+1, start, mid, l, r) + st.queryRange(2*node+2, mid+1, end, l, r)
}
