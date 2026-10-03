// Red-Black Tree — self-balancing BST.
package datastructures

import "fmt"

type color bool

const (
	red   color = true
	black color = false
)

type rbNode struct {
	key    int
	color  color
	left   *rbNode
	right  *rbNode
	parent *rbNode
}

type RBTree struct {
	root *rbNode
	nil  *rbNode
	size int
}

func NewRBTree() *RBTree {
	nil := &rbNode{color: black}
	return &RBTree{nil: nil, root: nil}
}

func (t *RBTree) Insert(key int) {
	z := &rbNode{key: key, color: red, left: t.nil, right: t.nil}
	var y *rbNode = t.nil
	x := t.root
	for x != t.nil {
		y = x
		if z.key < x.key {
			x = x.left
		} else {
			x = x.right
		}
	}
	z.parent = y
	if y == t.nil {
		t.root = z
	} else if z.key < y.key {
		y.left = z
	} else {
		y.right = z
	}
	t.size++
	t.insertFixup(z)
}

func (t *RBTree) leftRotate(x *rbNode) {
	y := x.right
	x.right = y.left
	if y.left != t.nil {
		y.left.parent = x
	}
	y.parent = x.parent
	if x.parent == t.nil {
		t.root = y
	} else if x == x.parent.left {
		x.parent.left = y
	} else {
		x.parent.right = y
	}
	y.left = x
	x.parent = y
}

func (t *RBTree) rightRotate(x *rbNode) {
	y := x.left
	x.left = y.right
	if y.right != t.nil {
		y.right.parent = x
	}
	y.parent = x.parent
	if x.parent == t.nil {
		t.root = y
	} else if x == x.parent.right {
		x.parent.right = y
	} else {
		x.parent.left = y
	}
	y.right = x
	x.parent = y
}

func (t *RBTree) insertFixup(z *rbNode) {
	for z.parent.color == red {
		if z.parent == z.parent.parent.left {
			y := z.parent.parent.right
			if y.color == red {
				z.parent.color = black
				y.color = black
				z.parent.parent.color = red
				z = z.parent.parent
			} else {
				if z == z.parent.right {
					z = z.parent
					t.leftRotate(z)
				}
				z.parent.color = black
				z.parent.parent.color = red
				t.rightRotate(z.parent.parent)
			}
		} else {
			y := z.parent.parent.left
			if y.color == red {
				z.parent.color = black
				y.color = black
				z.parent.parent.color = red
				z = z.parent.parent
			} else {
				if z == z.parent.left {
					z = z.parent
					t.rightRotate(z)
				}
				z.parent.color = black
				z.parent.parent.color = red
				t.leftRotate(z.parent.parent)
			}
		}
		if z == t.root {
			break
		}
	}
	t.root.color = black
}

func (t *RBTree) Search(key int) bool {
	node := t.root
	for node != t.nil {
		if key == node.key {
			return true
		}
		if key < node.key {
			node = node.left
		} else {
			node = node.right
		}
	}
	return false
}

func (t *RBTree) Inorder() []int {
	result := []int{}
	t.inorder(t.root, &result)
	return result
}

func (t *RBTree) inorder(node *rbNode, result *[]int) {
	if node == t.nil {
		return
	}
	t.inorder(node.left, result)
	*result = append(*result, node.key)
	t.inorder(node.right, result)
}

func (t *RBTree) Len() int { return t.size }

func (t *RBTree) VerifyInvariants() error {
	if t.root.color == red {
		return fmt.Errorf("root is red")
	}
	blackCount := -1
	return t.verify(t.root, 0, &blackCount)
}

func (t *RBTree) verify(node *rbNode, blackSoFar int, blackCount *int) error {
	if node == t.nil {
		if *blackCount == -1 {
			*blackCount = blackSoFar
		} else if blackSoFar != *blackCount {
			return fmt.Errorf("black height mismatch: %d vs %d", blackSoFar, *blackCount)
		}
		return nil
	}
	if node.color == red {
		if node.left.color == red || node.right.color == red {
			return fmt.Errorf("red node %d has red child", node.key)
		}
	}
	if node.color == black {
		blackSoFar++
	}
	if err := t.verify(node.left, blackSoFar, blackCount); err != nil {
		return err
	}
	return t.verify(node.right, blackSoFar, blackCount)
}
