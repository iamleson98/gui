// Fenwick Tree (Binary Indexed Tree).
package datastructures

type FenwickTree struct {
	tree []int64
	n    int
}

func NewFenwickTree(n int) *FenwickTree {
	return &FenwickTree{tree: make([]int64, n+1), n: n}
}

func NewFenwickTreeFrom(arr []int64) *FenwickTree {
	n := len(arr)
	ft := &FenwickTree{tree: make([]int64, n+1), n: n}
	for i := 0; i < n; i++ {
		ft.tree[i+1] = arr[i]
	}
	for i := 1; i <= n; i++ {
		p := i + (i & -i)
		if p <= n {
			ft.tree[p] += ft.tree[i]
		}
	}
	return ft
}

func (ft *FenwickTree) Update(i int, delta int64) {
	for i++; i <= ft.n; i += i & -i {
		ft.tree[i] += delta
	}
}

func (ft *FenwickTree) Query(i int) int64 {
	sum := int64(0)
	for i++; i > 0; i -= i & -i {
		sum += ft.tree[i]
	}
	return sum
}

func (ft *FenwickTree) QueryRange(l, r int) int64 {
	if l > r {
		return 0
	}
	return ft.Query(r) - ft.Query(l-1)
}
