// Question #117: Binary Search Tree (Basic)
// Category: Data Structures | Difficulty: Hard
// Concepts: BST, inorder, insert/delete, balanced
// Description: Implement an unbalanced BST with insert, delete, and search.
package datastructures

// Binary Search Tree (Basic)
// Implements a data structure for question #117.
type Q117_BinarySearchTreeBasic struct {
        data map[int]int
        size int
}

// NewQ117_BinarySearchTreeBasic creates a new instance.
func NewQ117_BinarySearchTreeBasic() *Q117_BinarySearchTreeBasic {
        return &Q117_BinarySearchTreeBasic{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q117_BinarySearchTreeBasic) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q117_BinarySearchTreeBasic) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q117_BinarySearchTreeBasic) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q117_BinarySearchTreeBasic) Len() int { return d.size }
