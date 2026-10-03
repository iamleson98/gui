// Question #275: Optimistic Concurrency Control
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: OCC, validation, read/write set, conflict
// Description: Validate transactions at commit time using read and write sets instead of locks.
package sql


// Optimistic Concurrency Control
// Implements a database design pattern for question #275.

// Q275_OptimisticConcurrencyControl represents the database schema/concept.
type Q275_OptimisticConcurrencyControl struct {
        tables map[string][]string
}

// NewQ275_OptimisticConcurrencyControl initializes the schema.
func NewQ275_OptimisticConcurrencyControl() *Q275_OptimisticConcurrencyControl {
        return &Q275_OptimisticConcurrencyControl{tables: make(map[string][]string)}
}
