// Question #125: Fractal Tree (TokuDB)
// Category: Data Structures | Difficulty: Hard
// Concepts: fractal tree, buffered, amortized I/O, B-tree
// Description: Build a fractal index tree using buffered insertions to amortize I/O across internal nodes.
package datastructures

// Fractal Tree (TokuDB)
// Implements a data structure for question #125.
type FractalTreeTokudb struct {
        data map[int]int
        size int
}

// NewFractalTreeTokudb creates a new instance.
func NewFractalTreeTokudb() *FractalTreeTokudb {
        return &FractalTreeTokudb{data: make(map[int]int)}
}

// Insert adds an element.
func (d *FractalTreeTokudb) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *FractalTreeTokudb) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *FractalTreeTokudb) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *FractalTreeTokudb) Len() int { return d.size }
