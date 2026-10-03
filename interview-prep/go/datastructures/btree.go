// B-Tree — balanced search tree with multi-way nodes.
package datastructures

import "sort"

const btreeMinDegree = 4

type btreeNode struct {
	keys     []int
	children []*btreeNode
	leaf     bool
}

type BTree struct {
	root *btreeNode
	t    int
}

func NewBTree() *BTree {
	return &BTree{t: btreeMinDegree}
}

func (bt *BTree) Insert(key int) {
	if bt.root == nil {
		bt.root = &btreeNode{leaf: true}
	}
	r := bt.root
	if len(r.keys) >= 2*bt.t-1 {
		s := &btreeNode{leaf: false}
		s.children = append(s.children, r)
		bt.splitChild(s, 0)
		bt.root = s
	}
	bt.insertNonFull(bt.root, key)
}

func (bt *BTree) splitChild(parent *btreeNode, i int) {
	t := bt.t
	child := parent.children[i]
	mid := t - 1
	newNode := &btreeNode{leaf: child.leaf}
	newNode.keys = append(newNode.keys, child.keys[t:]...)
	if !child.leaf {
		newNode.children = append(newNode.children, child.children[t:]...)
		child.children = child.children[:t]
	}
	midKey := child.keys[mid]
	child.keys = child.keys[:mid]
	parent.keys = append(parent.keys, 0)
	copy(parent.keys[i+1:], parent.keys[i:])
	parent.keys[i] = midKey
	parent.children = append(parent.children, nil)
	copy(parent.children[i+2:], parent.children[i+1:])
	parent.children[i+1] = newNode
}

func (bt *BTree) insertNonFull(node *btreeNode, key int) {
	i := len(node.keys) - 1
	if node.leaf {
		node.keys = append(node.keys, 0)
		for i >= 0 && key < node.keys[i] {
			node.keys[i+1] = node.keys[i]
			i--
		}
		node.keys[i+1] = key
	} else {
		for i >= 0 && key < node.keys[i] {
			i--
		}
		i++
		if len(node.children[i].keys) >= 2*bt.t-1 {
			bt.splitChild(node, i)
			if key > node.keys[i] {
				i++
			}
		}
		bt.insertNonFull(node.children[i], key)
	}
}

func (bt *BTree) Search(key int) bool {
	return bt.search(bt.root, key)
}

func (bt *BTree) search(node *btreeNode, key int) bool {
	if node == nil {
		return false
	}
	i := sort.Search(len(node.keys), func(i int) bool { return node.keys[i] >= key })
	if i < len(node.keys) && node.keys[i] == key {
		return true
	}
	if node.leaf {
		return false
	}
	return bt.search(node.children[i], key)
}

func (bt *BTree) RangeQuery(lo, hi int) []int {
	result := []int{}
	bt.rangeQuery(bt.root, lo, hi, &result)
	return result
}

func (bt *BTree) rangeQuery(node *btreeNode, lo, hi int, result *[]int) {
	if node == nil {
		return
	}
	for i := 0; i < len(node.keys); i++ {
		if !node.leaf {
			bt.rangeQuery(node.children[i], lo, hi, result)
		}
		if node.keys[i] >= lo && node.keys[i] <= hi {
			*result = append(*result, node.keys[i])
		}
	}
	if !node.leaf {
		bt.rangeQuery(node.children[len(node.keys)], lo, hi, result)
	}
}
