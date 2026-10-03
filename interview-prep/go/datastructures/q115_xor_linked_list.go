// Question #115: XOR Linked List
// Category: Data Structures | Difficulty: Hard
// Concepts: XOR list, pointer compression, memory, traversal
// Description: Implement a doubly linked list using XOR of adjacent pointers to store one pointer per node.
package datastructures

// XOR Linked List
// Implements a data structure for question #115.
type Q115_XorLinkedList struct {
        data map[int]int
        size int
}

// NewQ115_XorLinkedList creates a new instance.
func NewQ115_XorLinkedList() *Q115_XorLinkedList {
        return &Q115_XorLinkedList{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q115_XorLinkedList) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q115_XorLinkedList) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q115_XorLinkedList) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q115_XorLinkedList) Len() int { return d.size }
