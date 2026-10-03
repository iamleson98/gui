// Question #232: Composite Primary Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite key, primary key, indexes, foreign key
// Description: Design composite primary keys and reason about their impact on indexes and foreign keys.
package sql

import "fmt"

// Composite Primary Keys
// Implements a database design pattern for question #232.

// CompositePrimaryKeys represents the database schema/concept.
type CompositePrimaryKeys struct {
        tables map[string][]string
}

// NewCompositePrimaryKeys initializes the schema.
func NewCompositePrimaryKeys() *CompositePrimaryKeys {
        return &CompositePrimaryKeys{tables: make(map[string][]string)}
}
