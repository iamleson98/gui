// Question #282: CQRS
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: CQRS, command, query, separation
// Description: Separate command (write) and query (read) models to optimize each independently.
package sql

import "fmt"

// CQRS
// Implements a database design pattern for question #282.

// Cqrs represents the database schema/concept.
type Cqrs struct {
        tables map[string][]string
}

// NewCqrs initializes the schema.
func NewCqrs() *Cqrs {
        return &Cqrs{tables: make(map[string][]string)}
}
