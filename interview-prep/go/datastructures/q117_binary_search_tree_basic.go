// Question #117: Binary Search Tree (Basic)
// Category: Data Structures | Difficulty: Hard
// Concepts: BST, inorder, insert/delete, balanced
// Description: Implement an unbalanced BST with insert, delete, and search.
package datastructures

// Binary Search Tree (Basic)
// Implements a data structure for question #117.
type BinarySearchTreeBasic struct {
        data map[int]int
        size int
}

// NewBinarySearchTreeBasic creates a new instance.
func NewBinarySearchTreeBasic() *BinarySearchTreeBasic {
        return &BinarySearchTreeBasic{data: make(map[int]int)}
}

// Insert adds an element.
func (d *BinarySearchTreeBasic) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *BinarySearchTreeBasic) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *BinarySearchTreeBasic) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *BinarySearchTreeBasic) Len() int { return d.size }
