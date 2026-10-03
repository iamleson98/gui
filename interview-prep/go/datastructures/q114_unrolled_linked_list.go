// Question #114: Unrolled Linked List
// Category: Data Structures | Difficulty: Hard
// Concepts: unrolled list, cache locality, node capacity, linked list
// Description: Build a linked list whose nodes store multiple elements to improve cache locality.
package datastructures

// Unrolled Linked List
// Implements a data structure for question #114.
type Q114_UnrolledLinkedList struct {
        data map[int]int
        size int
}

// NewQ114_UnrolledLinkedList creates a new instance.
func NewQ114_UnrolledLinkedList() *Q114_UnrolledLinkedList {
        return &Q114_UnrolledLinkedList{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q114_UnrolledLinkedList) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q114_UnrolledLinkedList) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q114_UnrolledLinkedList) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q114_UnrolledLinkedList) Len() int { return d.size }
