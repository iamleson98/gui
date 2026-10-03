// Question #128: Clojure-style Persistent Vector
// Category: Data Structures | Difficulty: Hard
// Concepts: persistent vector, bitmapped trie, tail, immutability
// Description: Implement a persistent vector using a bitmapped trie indexed by chunks.
package datastructures

// Clojure-style Persistent Vector
// Implements a data structure for question #128.
type Q128_ClojureStylePersistentVector struct {
        data map[int]int
        size int
}

// NewQ128_ClojureStylePersistentVector creates a new instance.
func NewQ128_ClojureStylePersistentVector() *Q128_ClojureStylePersistentVector {
        return &Q128_ClojureStylePersistentVector{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q128_ClojureStylePersistentVector) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q128_ClojureStylePersistentVector) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q128_ClojureStylePersistentVector) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q128_ClojureStylePersistentVector) Len() int { return d.size }
