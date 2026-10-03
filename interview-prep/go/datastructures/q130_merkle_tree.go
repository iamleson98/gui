// Question #130: Merkle Tree
// Category: Data Structures | Difficulty: Hard
// Concepts: Merkle tree, hash, inclusion proof, tamper detection
// Description: Implement a Merkle tree of content hashes supporting inclusion proofs and tamper detection.
package datastructures

// Merkle Tree
// Implements a data structure for question #130.
type Q130_MerkleTree struct {
        data map[int]int
        size int
}

// NewQ130_MerkleTree creates a new instance.
func NewQ130_MerkleTree() *Q130_MerkleTree {
        return &Q130_MerkleTree{data: make(map[int]int)}
}

// Insert adds an element.
func (d *Q130_MerkleTree) Insert(key, val int) {
        d.data[key] = val
        d.size++
}

// Search looks up an element.
func (d *Q130_MerkleTree) Search(key int) (int, bool) {
        v, ok := d.data[key]
        return v, ok
}

// Delete removes an element.
func (d *Q130_MerkleTree) Delete(key int) bool {
        if _, ok := d.data[key]; ok {
                delete(d.data, key)
                d.size--
                return true
        }
        return false
}

// Len returns the number of elements.
func (d *Q130_MerkleTree) Len() int { return d.size }
