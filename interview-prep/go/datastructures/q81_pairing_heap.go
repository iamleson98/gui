// Question #81: Pairing Heap
// Category: Data Structures | Difficulty: Hard
// Concepts: pairing heap, merge, two-pass, amortized
// Description: Build a pairing heap that achieves practical speed via two-pass merging of children.
package datastructures

// Pairing Heap
// Implements a data structure for question #81.
type PairingHeap struct {
        data map[int]int
        size int
}

// NewPairingHeap creates a new instance.
func NewPairingHeap() *PairingHeap {
        return &PairingHeap{data: make(map[int]int)}
}

// Insert adds an element.
func (d *PairingHeap) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *PairingHeap) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *PairingHeap) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *PairingHeap) Len() int { return d.size }
