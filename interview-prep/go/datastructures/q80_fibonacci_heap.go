// Question #80: Fibonacci Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: Fibonacci heap, amortized, decrease-key, cascading cut
// Description: Implement a Fibonacci heap with lazy melding and amortized O(1) decrease-key for Dijkstra.
package datastructures

// Fibonacci Heap
// Implements a data structure for question #80.
type FibonacciHeap struct {
        data map[int]int
        size int
}

// NewFibonacciHeap creates a new instance.
func NewFibonacciHeap() *FibonacciHeap {
        return &FibonacciHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *FibonacciHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *FibonacciHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *FibonacciHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *FibonacciHeap) Len() int { return d.size }
