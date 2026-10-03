// Question #274: Gap Locks and Next-Key Locking
// Category: SQL & Database Design | Difficulty: Hard
// Concepts: gap lock, next-key, phantom, InnoDB
// Description: Prevent phantom reads with gap and next-key locking in InnoDB repeatable-read.
package sql

import "fmt"

// Gap Locks and Next-Key Locking
// Implements a database design pattern for question #274.

// GapLocksAndNextKeyLocking represents the database schema/concept.
type GapLocksAndNextKeyLocking struct {
        tables map[string][]string
}

// NewGapLocksAndNextKeyLocking initializes the schema.
func NewGapLocksAndNextKeyLocking() *GapLocksAndNextKeyLocking {
        return &GapLocksAndNextKeyLocking{tables: make(map[string][]string)}
}
