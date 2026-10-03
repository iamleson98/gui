// Question #276: Isolation Levels
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: isolation, read committed, repeatable read, serializable
// Description: Contrast read-uncommitted, read-committed, repeatable-read, and serializable isolation.
package sql


// Isolation Levels
// Implements a database design pattern for question #276.

// Q276_IsolationLevels represents the database schema/concept.
type Q276_IsolationLevels struct {
        tables map[string][]string
}

// NewQ276_IsolationLevels initializes the schema.
func NewQ276_IsolationLevels() *Q276_IsolationLevels {
        return &Q276_IsolationLevels{tables: make(map[string][]string)}
}
