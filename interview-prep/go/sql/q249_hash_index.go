// Question #249: Hash Index
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: hash index, equality, no range, buckets
// Description: Use hash indexes for equality lookups and note their unsuitability for range queries.
package sql


// Hash Index
// Implements a database design pattern for question #249.

// Q249_HashIndex represents the database schema/concept.
type Q249_HashIndex struct {
        tables map[string][]string
}

// NewQ249_HashIndex initializes the schema.
func NewQ249_HashIndex() *Q249_HashIndex {
        return &Q249_HashIndex{tables: make(map[string][]string)}
}
