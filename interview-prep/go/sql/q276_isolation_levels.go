// Question #276: Isolation Levels
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: isolation, read committed, repeatable read, serializable
// Description: Contrast read-uncommitted, read-committed, repeatable-read, and serializable isolation.
package sql

import "fmt"

// Isolation Levels
// Implements a database design pattern for question #276.

// IsolationLevels represents the database schema/concept.
type IsolationLevels struct {
        tables map[string][]string
}

// NewIsolationLevels initializes the schema.
func NewIsolationLevels() *IsolationLevels {
        return &IsolationLevels{tables: make(map[string][]string)}
}
