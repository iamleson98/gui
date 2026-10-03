// Question #232: Composite Primary Keys
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: composite key, primary key, indexes, foreign key
// Description: Design composite primary keys and reason about their impact on indexes and foreign keys.
package sql


// Composite Primary Keys
// Implements a database design pattern for question #232.

// Q232_CompositePrimaryKeys represents the database schema/concept.
type Q232_CompositePrimaryKeys struct {
        tables map[string][]string
}

// NewQ232_CompositePrimaryKeys initializes the schema.
func NewQ232_CompositePrimaryKeys() *Q232_CompositePrimaryKeys {
        return &Q232_CompositePrimaryKeys{tables: make(map[string][]string)}
}
