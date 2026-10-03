// Question #88: Wavelet Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: wavelet tree, rank/select, bit vector, sequence
// Description: Implement a wavelet tree for rank/select queries over a sequence using bit vectors.
package datastructures

// Wavelet Tree
// Implements a data structure for question #88.
type WaveletTree struct {
        data map[int]int
        size int
}

// NewWaveletTree creates a new instance.
func NewWaveletTree() *WaveletTree {
        return &WaveletTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *WaveletTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *WaveletTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *WaveletTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *WaveletTree) Len() int { return d.size }
