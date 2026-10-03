// Question #249: Hash Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: hash index, equality, no range, buckets
// Description: Use hash indexes for equality lookups and note their unsuitability for range queries.
package sql

import "fmt"

// Hash Index
// Implements a database design pattern for question #249.

// HashIndex represents the database schema/concept.
type HashIndex struct {
        tables map[string][]string
}

// NewHashIndex initializes the schema.
func NewHashIndex() *HashIndex {
        return &HashIndex{tables: make(map[string][]string)}
}
