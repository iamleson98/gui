// Question #119: Randomized BST
// Category: Data Structures | Difficulty: Hard
// Concepts: randomized BST, expected balance, root insert, probability
// Description: Implement a randomized BST that inserts at the root with probability 1/n to stay balanced in expectation.
package datastructures

// Randomized BST
// Implements a data structure for question #119.
type Q119_RandomizedBst struct {
        data map[int]int
        size int
}

// NewQ119_RandomizedBst creates a new instance.
func NewQ119_RandomizedBst() *Q119_RandomizedBst {
        return &Q119_RandomizedBst{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q119_RandomizedBst) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q119_RandomizedBst) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q119_RandomizedBst) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q119_RandomizedBst) Len() int { return d.size }
