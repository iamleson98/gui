// Question #275: Optimistic Concurrency Control
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: OCC, validation, read/write set, conflict
// Description: Validate transactions at commit time using read and write sets instead of locks.
package sql

import "fmt"

// Optimistic Concurrency Control
// Implements a database design pattern for question #275.

// OptimisticConcurrencyControl represents the database schema/concept.
type OptimisticConcurrencyControl struct {
        tables map[string][]string
}

// NewOptimisticConcurrencyControl initializes the schema.
func NewOptimisticConcurrencyControl() *OptimisticConcurrencyControl {
        return &OptimisticConcurrencyControl{tables: make(map[string][]string)}
}
